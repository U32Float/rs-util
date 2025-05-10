use std::panic::Location;

use tracing::{Level, event};

// -----------------------------------------------------------------------------

#[macro_export]
macro_rules! const_assert {
    ($cond:expr, $($arg:tt)*) => {
        const {
            if !$cond {
                panic!($($arg)*);
            }
        }
    };
}

#[macro_export]
macro_rules! const_assert_eq {
    ($left:expr, $right: expr, $($arg:tt)*) => {
        const {
            if $left != $right {
                panic!($($arg)*);
            }
        }
    };
}

#[macro_export]
macro_rules! const_assert_ne {
    ($left:expr, $right: expr, $($arg:tt)*) => {
        const {
            if $left == $right {
                panic!($($arg)*);
            }
        }
    };
}

// -----------------------------------------------------------------------------

#[macro_export]
macro_rules! debug_panic {
    ( $($fmt_arg:tt)* ) => {
        if cfg!(debug_assertions) {
            panic!( $($fmt_arg)* );
        } else {
            let backtrace = std::backtrace::Backtrace::capture();
            tracing::error!("{}\n{:?}", format_args!($($fmt_arg)*), backtrace);
        }
    };
}

fn log_error_with_caller<E>(caller: core::panic::Location<'_>, error: E, level: tracing::Level)
where
    E: std::fmt::Debug,
{
    #[cfg(not(target_os = "windows"))]
    let file = caller.file();
    #[cfg(target_os = "windows")]
    let file = caller.file().replace('\\', "/");
    let target = file.split('/').nth(1);

    macro_rules! log {
        ($e: expr) => {
            event!($e, target=  target.unwrap_or(""), file = caller.file(), line = caller.line(), error = ?error)
        };
    }

    match level {
        Level::ERROR => log!(Level::ERROR),
        Level::WARN => log!(Level::WARN),
        Level::INFO => log!(Level::INFO),
        Level::DEBUG => log!(Level::DEBUG),
        Level::TRACE => log!(Level::TRACE),
    }
}

pub trait ResultExt<E> {
    type Ok;

    fn log_err(self) -> Option<Self::Ok>;
    /// Assert that this result should never be an error in development or tests.
    fn debug_assert_ok(self, reason: &str) -> Self;
    fn warn_on_err(self) -> Option<Self::Ok>;
}

impl<T, E> ResultExt<E> for Result<T, E>
where
    E: std::fmt::Debug,
{
    type Ok = T;

    #[track_caller]
    fn log_err(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(error) => {
                log_error_with_caller(*Location::caller(), error, Level::ERROR);
                None
            }
        }
    }

    #[track_caller]
    fn debug_assert_ok(self, reason: &str) -> Self {
        if let Err(error) = &self {
            debug_panic!("{reason} - {error:?}");
        }
        self
    }

    #[track_caller]
    fn warn_on_err(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(error) => {
                log_error_with_caller(*Location::caller(), error, Level::WARN);
                None
            }
        }
    }
}

// -----------------------------------------------------------------------------

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pub trait TryFutureExt {
    fn log_err(self) -> LogErrorFuture<Self>
    where
        Self: Sized;

    fn log_tracked_err(self, location: core::panic::Location<'static>) -> LogErrorFuture<Self>
    where
        Self: Sized;

    fn warn_on_err(self) -> LogErrorFuture<Self>
    where
        Self: Sized;
    fn unwrap(self) -> UnwrapFuture<Self>
    where
        Self: Sized;
}

impl<F, T, E> TryFutureExt for F
where
    F: Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    #[track_caller]
    fn log_err(self) -> LogErrorFuture<Self>
    where
        Self: Sized,
    {
        let location = Location::caller();
        LogErrorFuture(self, tracing::Level::ERROR, *location)
    }

    fn log_tracked_err(self, location: core::panic::Location<'static>) -> LogErrorFuture<Self>
    where
        Self: Sized,
    {
        LogErrorFuture(self, tracing::Level::ERROR, location)
    }

    #[track_caller]
    fn warn_on_err(self) -> LogErrorFuture<Self>
    where
        Self: Sized,
    {
        let location = Location::caller();
        LogErrorFuture(self, tracing::Level::WARN, *location)
    }

    fn unwrap(self) -> UnwrapFuture<Self>
    where
        Self: Sized,
    {
        UnwrapFuture(self)
    }
}

#[must_use]
pub struct LogErrorFuture<F>(F, tracing::Level, core::panic::Location<'static>);

impl<F, T, E> Future for LogErrorFuture<F>
where
    F: Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    type Output = Option<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let level = self.1;
        let location = self.2;
        let inner = unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().0) };
        match inner.poll(cx) {
            Poll::Ready(output) => Poll::Ready(match output {
                Ok(output) => Some(output),
                Err(error) => {
                    log_error_with_caller(location, error, level);
                    None
                }
            }),
            Poll::Pending => Poll::Pending,
        }
    }
}

pub struct UnwrapFuture<F>(F);

impl<F, T, E> Future for UnwrapFuture<F>
where
    F: Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let inner = unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().0) };
        match inner.poll(cx) {
            Poll::Ready(result) => Poll::Ready(result.unwrap()),
            Poll::Pending => Poll::Pending,
        }
    }
}
