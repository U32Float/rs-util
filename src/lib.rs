#![feature(cell_update)]
#![feature(str_from_utf16_endian)]

mod arc_cow;
pub use arc_cow::ArcCow;

mod shared_string;
use parking_lot::Mutex;
pub use shared_string::SharedString;

mod log;
pub use log::*;

mod option;
pub use option::*;

mod io;
pub use io::*;

mod range;
pub use range::*;

mod toggle;
pub use toggle::*;

// -----------------------------------------------------------------------------

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

// -----------------------------------------------------------------------------

pub trait LockExt<T> {
    fn read<R>(&self, f: impl FnOnce(&T) -> R) -> R;
    fn write<R>(&self, f: impl FnOnce(&mut T) -> R) -> R;
}

impl<T> LockExt<T> for Mutex<T> {
    fn read<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        let guard = self.lock();
        f(&*guard)
    }

    fn write<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut guard = self.lock();
        f(&mut *guard)
    }
}

// -----------------------------------------------------------------------------

#[inline(always)]
pub fn post_inc<T: From<u8> + std::ops::AddAssign<T> + Copy>(value: &mut T) -> T {
    let prev = *value;
    *value += T::from(1);
    prev
}

// -----------------------------------------------------------------------------

#[macro_export]
macro_rules! implies {
    ($p:expr => $q:expr) => {
        ($p && $q) || !$p
    };
}

// -----------------------------------------------------------------------------

#[macro_export]
macro_rules! with_clone {
    ($i:ident, move ||$l:expr) => {{
        let $i = $i.clone();
        move || {
            $l
        }
    }};
    ($i:ident, move |$($k:pat_param),*|$l:expr) => {{
        let $i = $i.clone();
        move |$( $k ),*| {
            $l
        }
    }};

    (($($i:ident),+), move ||$l:expr) => {{
        let ($($i),+) = ($($i.clone()),+);
        move || {
            $l
        }
    }};
    (($($i:ident),+), move |$($k:pat_param),*|$l:expr) => {{
        let ($($i),+) = ($($i.clone()),+);
        move |$( $k ),*| {
            $l
        }
    }};
}

mod test_with_clone {

    // If this test compiles, it works
    #[test]
    fn test() {
        let x = "String".to_string();
        let y = std::sync::Arc::new(5);

        fn no_arg(f: impl FnOnce()) {
            f()
        }

        no_arg(with_clone!(x, move || {
            drop(x);
        }));

        no_arg(with_clone!((x, y), move || {
            drop(x);
            drop(y);
        }));

        fn one_arg(f: impl FnOnce(usize)) {
            f(1)
        }

        one_arg(with_clone!(x, move |_| {
            drop(x);
        }));
        one_arg(with_clone!((x, y), move |b| {
            drop(x);
            drop(y);
            println!("{}", b);
        }));

        fn two_arg(f: impl FnOnce(usize, bool)) {
            f(5, true)
        }

        two_arg(with_clone!((x, y), move |a, b| {
            drop(x);
            drop(y);
            println!("{}{}", a, b)
        }));
        two_arg(with_clone!((x, y), move |a, _| {
            drop(x);
            drop(y);
            println!("{}", a)
        }));
        two_arg(with_clone!((x, y), move |_, b| {
            drop(x);
            drop(y);
            println!("{}", b)
        }));

        struct Example {
            z: usize,
        }

        fn destructuring_example(f: impl FnOnce(Example)) {
            f(Example { z: 10 })
        }

        destructuring_example(with_clone!(x, move |Example { z }| {
            drop(x);
            println!("{}", z);
        }));

        let a_long_variable_1 = "".to_string();
        let a_long_variable_2 = "".to_string();
        let a_long_variable_3 = "".to_string();
        let a_long_variable_4 = "".to_string();
        two_arg(with_clone!(
            (
                x,
                y,
                a_long_variable_1,
                a_long_variable_2,
                a_long_variable_3,
                a_long_variable_4
            ),
            move |a, b| {
                drop(x);
                drop(y);
                drop(a_long_variable_1);
                drop(a_long_variable_2);
                drop(a_long_variable_3);
                drop(a_long_variable_4);
                println!("{}{}", a, b)
            }
        ));

        fn single_expression_body(f: impl FnOnce(usize) -> usize) -> usize {
            f(20)
        }

        let _result = single_expression_body(with_clone!(y, move |z| *y + z));

        // Explicitly move all variables
        drop(x);
        drop(y);
        drop(a_long_variable_1);
        drop(a_long_variable_2);
        drop(a_long_variable_3);
        drop(a_long_variable_4);
    }
}
