use std::sync::{Arc, Condvar, Mutex};

/// A task that can be awaited in an async context or waited for in a sync context without `block_on`.
pub struct Task<T> {
    awaitable: std::pin::Pin<Box<dyn Future<Output = T> + Send>>,
    pair: Option<Arc<(Mutex<Option<T>>, Condvar)>>,
}

impl<T> Future for Task<T> {
    type Output = T;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.pair.take(); // Decrease reference count to prevent panic.
        self.awaitable.as_mut().poll(cx)
    }
}

impl<T> Task<T> {
    /// Wait for the task to complete and return the result in a sync context.
    pub fn wait_sync(self) -> T {
        let (lock, cvar) = &*self.pair.unwrap();
        let mut guard = lock.lock().unwrap();
        while guard.is_none() {
            guard = cvar.wait(guard).unwrap(); // blocks until notified
        }
        guard.take().unwrap()
    }
}

pub fn spawn_task<T, F, S>(future: F, schedule: S) -> (async_task::Runnable, Task<T>)
where
    T: Send + 'static,
    F: Future<Output = T> + Send + 'static,
    S: async_task::Schedule + Send + Sync + 'static,
{
    let pair = Arc::new((Mutex::new(None), Condvar::new()));
    let pair_task = Arc::clone(&pair);
    let pair_sync = Arc::clone(&pair);

    let (runnable, task) = async_task::spawn(
        async move {
            let val = future.await;
            let (lock, cvar) = &*pair_task;
            *lock.lock().unwrap() = Some(val);
            cvar.notify_all(); // Wake up any waiting sync threads
        },
        schedule,
    );

    let awaitable = async move {
        task.await;
        let (lock, _) = Arc::try_unwrap(pair)
            .ok()
            .expect("Multiple references remain");
        lock.into_inner().unwrap().expect("Result not set")
    };

    (
        runnable,
        Task {
            awaitable: Box::pin(awaitable),
            pair: Some(pair_sync),
        },
    )
}
