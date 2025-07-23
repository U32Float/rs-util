#![feature(str_from_utf16_endian)]

use parking_lot::Mutex;

#[cfg(feature = "macros")]
pub mod macros;

pub_use! {
    #[cfg(feature = "id")]
    mod id;
    #[cfg(feature = "arc_cow")]
    mod arc_cow;
    #[cfg(feature = "shared_string")]
    mod shared_string;
    #[cfg(feature = "log")]
    mod log;
    #[cfg(feature = "option")]
    mod option;
    #[cfg(feature = "io")]
    mod io;
    #[cfg(feature = "range")]
    mod range;
    #[cfg(feature = "toggle")]
    mod toggle;
    #[cfg(feature = "cell")]
    mod cell;
    #[cfg(feature = "async")]
    mod task;
    #[cfg(feature = "builder")]
    mod builder;
    #[cfg(feature = "retainable")]
    mod retainable;
    #[cfg(feature = "copy_on_write")]
    mod copy_on_write;
    #[cfg(feature = "deferred")]
    mod deferred;
}

// -----------------------------------------------------------------------------

#[macro_export]
macro_rules! pub_use {
    ($(
        $(#[$attr:meta])?
        mod $name:ident $(;)?
    )*) => {
        $(
            $(#[$attr])?
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
