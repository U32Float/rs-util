#![feature(cell_update)]
#![feature(str_from_utf16_endian)]

use parking_lot::Mutex;

pub mod id;
pub mod macros;

pub_use!(mod arc_cow);
pub_use!(mod shared_string);
pub_use!(mod log);
pub_use!(mod option);
pub_use!(mod io);
pub_use!(mod range);
pub_use!(mod toggle);
pub_use!(mod cell);

// -----------------------------------------------------------------------------

#[macro_export]
macro_rules! pub_use {
    (mod $name:ident) => {
        mod $name;
        pub use $name::*;
    };
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
