pub trait OptionExt<T> {
    fn unwrap_ref(&self) -> &T;
    fn unwrap_mut(&mut self) -> &mut T;
}

impl<T> OptionExt<T> for Option<T> {
    fn unwrap_ref(&self) -> &T {
        self.as_ref().unwrap()
    }

    fn unwrap_mut(&mut self) -> &mut T {
        self.as_mut().unwrap()
    }
}
