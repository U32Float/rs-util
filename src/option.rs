pub trait OptionExt<T> {
    fn unwrap_ref(&self) -> &T;
    fn unwrap_mut(&mut self) -> &mut T;
    fn map_ref<U>(&self, f: impl FnOnce(&T) -> U) -> Option<U>;
    fn map_mut<U>(&mut self, f: impl FnOnce(&mut T) -> U) -> Option<U>;
}

impl<T> OptionExt<T> for Option<T> {
    fn unwrap_ref(&self) -> &T {
        self.as_ref().unwrap()
    }

    fn unwrap_mut(&mut self) -> &mut T {
        self.as_mut().unwrap()
    }

    fn map_ref<U>(&self, f: impl FnOnce(&T) -> U) -> Option<U> {
        self.as_ref().map(f)
    }

    fn map_mut<U>(&mut self, f: impl FnOnce(&mut T) -> U) -> Option<U> {
        self.as_mut().map(f)
    }
}
