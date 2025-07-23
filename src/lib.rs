#![feature(str_from_utf16_endian)]

use parking_lot::Mutex;

#[cfg(feature = "id")]
pub_use!(mod id);
#[cfg(feature = "macros")]
pub mod macros;

#[cfg(feature = "arc_cow")]
pub_use!(mod arc_cow);
#[cfg(feature = "shared_string")]
pub_use!(mod shared_string);
#[cfg(feature = "log")]
pub_use!(mod log);
#[cfg(feature = "option")]
pub_use!(mod option);
#[cfg(feature = "io")]
pub_use!(mod io);
#[cfg(feature = "range")]
pub_use!(mod range);
#[cfg(feature = "toggle")]
pub_use!(mod toggle);
#[cfg(feature = "cell")]
pub_use!(mod cell);
#[cfg(feature = "async")]
pub_use!(mod task);
#[cfg(feature = "builder")]
pub_use!(mod builder);
#[cfg(feature = "retainable")]
pub_use!(mod retainable);
#[cfg(feature = "copy_on_write")]
pub_use!(mod copy_on_write);
#[cfg(feature = "deferred")]
pub_use!(mod deferred);

// -----------------------------------------------------------------------------

#[macro_export]
macro_rules! pub_use {
    ($(mod $name:ident $(;)?)*) => {
        $(
            mod $name;
            pub use $name::*;
        )*
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
