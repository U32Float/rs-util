#[derive(Clone)]
/// An owned smart pointer that enables cheap cloning by sharing its data internally.
/// On mutation, the data is cloned to ensure unique ownership (copy-on-write).
///
/// Useful when cloning is common but mutation is rare.
pub struct CopyOnWrite<T>(std::sync::Arc<T>);

impl<T: Clone> CopyOnWrite<T> {
    #[inline(always)]
    pub fn ptr_eq(this: &Self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&this.0, &other.0)
    }

    #[inline(always)]
    pub fn new(value: T) -> Self {
        Self(std::sync::Arc::new(value))
    }

    #[inline(always)]
    pub fn read<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        f(&self.0)
    }

    #[inline(always)]
    pub fn write<R>(&mut self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut value = (*self.0).clone();
        let ret = f(&mut value);
        self.0 = std::sync::Arc::new(value);
        ret
    }
}

impl<T> AsRef<T> for CopyOnWrite<T> {
    #[inline(always)]
    fn as_ref(&self) -> &T {
        &self.0
    }
}
