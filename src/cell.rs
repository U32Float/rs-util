#[derive(Debug, Default)]
pub struct RcRefCell<T>(std::rc::Rc<std::cell::RefCell<T>>);

impl<T> Clone for RcRefCell<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> RcRefCell<T> {
    #[inline(always)]
    pub fn new(value: T) -> Self {
        Self(std::rc::Rc::new(std::cell::RefCell::new(value)))
    }

    #[inline(always)]
    pub fn downgrade(&self) -> WeakRefCell<T> {
        WeakRefCell(std::rc::Rc::downgrade(&self.0))
    }

    #[inline(always)]
    pub fn borrow(&self) -> std::cell::Ref<T> {
        self.0.borrow()
    }

    #[inline(always)]
    pub fn borrow_mut(&self) -> std::cell::RefMut<T> {
        self.0.borrow_mut()
    }

    #[inline(always)]
    pub fn take(&self) -> T
    where
        T: Default,
    {
        (&self.0).take()
    }
}

pub struct WeakRefCell<T>(std::rc::Weak<std::cell::RefCell<T>>);

impl<T> Clone for WeakRefCell<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> WeakRefCell<T> {
    #[inline(always)]
    pub fn upgrade(&self) -> Option<RcRefCell<T>> {
        self.0.upgrade().map(RcRefCell)
    }
}

#[derive(Debug, Default)]
pub struct RcCell<T: Copy>(std::rc::Rc<std::cell::Cell<T>>);

impl<T: Copy> Clone for RcCell<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Copy> RcCell<T> {
    #[inline(always)]
    pub fn new(value: T) -> Self {
        Self(std::rc::Rc::new(std::cell::Cell::new(value)))
    }

    #[inline(always)]
    pub fn get(&self) -> T {
        self.0.get()
    }

    #[inline(always)]
    pub fn replace(&self, val: T) -> T {
        self.0.replace(val)
    }

    #[inline(always)]
    pub fn set(&self, val: T) {
        self.0.set(val)
    }

    #[inline(always)]
    pub fn update(&self, f: impl FnOnce(T) -> T) {
        self.0.update(f)
    }

    #[inline(always)]
    pub fn swap(&self, other: &std::cell::Cell<T>) {
        self.0.swap(other)
    }
}
