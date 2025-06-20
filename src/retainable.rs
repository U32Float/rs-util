use std::ops::Sub;

/// A value that is retained until the last [`Subscription`] is dropped.
/// Dropping must be done manually after checking `self.retain()`.
pub struct Retainable<T> {
    ref_count: std::sync::Weak<()>,
    pub value: T,
}

impl<T> Retainable<T> {
    pub fn new(value: T) -> (Self, Subscription) {
        let ref_count = std::sync::Arc::new(());
        (
            Self {
                ref_count: std::sync::Arc::downgrade(&ref_count),
                value,
            },
            Subscription(ref_count),
        )
    }

    pub fn new_subscription(&self) -> Option<Subscription> {
        Some(Subscription(self.ref_count.upgrade()?))
    }
}

impl<T> Retainable<T> {
    pub fn retain(&self) -> bool {
        std::sync::Weak::strong_count(&self.ref_count) > 0
    }
}

impl<T> AsRef<T> for Retainable<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T> AsMut<T> for Retainable<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

#[derive(Clone)]
pub struct Subscription(std::sync::Arc<()>);

impl Subscription {
    pub fn detach(self) {
        std::mem::forget(self);
    }
}
