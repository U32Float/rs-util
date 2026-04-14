#![feature(str_from_utf16_endian)]
#![feature(hash_set_entry)]

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
    #[cfg(feature = "puffin")]
    mod profile;
}

#[cfg(feature = "macros")]
pub mod macros;

// -----------------------------------------------------------------------------

use std::{collections::hash_set::Entry, hash::Hash, sync::OnceLock};

use parking_lot::Mutex;
use rustc_hash::FxHashSet;

// -----------------------------------------------------------------------------

#[macro_export]
macro_rules! local_use {
    ($(
        $(#[$attr:meta])?
        $vis:vis mod $name:ident $(;)?
    )*) => {
        $(
            $(#[$attr])?
            $vis mod $name;
            use $name::*;
        )*
    };
}

#[macro_export]
macro_rules! crate_use {
    ($(
        $(#[$attr:meta])?
        $vis:vis mod $name:ident $(;)?
    )*) => {
        $(
            $(#[$attr])?
            $vis mod $name;
            pub(crate) use $name::*;
        )*
    };
}

#[macro_export]
macro_rules! pub_use {
    ($(
        $(#[$attr:meta])?
        $vis:vis mod $name:ident $(;)?
    )*) => {
        $(
            $(#[$attr])?
            $vis mod $name;
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

#[inline]
/// Hash the given value with a predictable hasher.
pub fn hash(value: impl std::hash::Hash) -> u64 {
    ahash::RandomState::with_seeds(1, 2, 3, 4).hash_one(value)
}

static COMPLETION_SET: OnceLock<Mutex<FxHashSet<u64>>> = OnceLock::new();

#[inline(always)]
/// Executes the given function only once for each unique `id`.
/// Subsequent calls with the same `id` will be ignored.
pub fn once<T>(id: impl Hash, f: impl FnOnce() -> T) -> Option<T> {
    let mut lock = COMPLETION_SET
        .get_or_init(|| Mutex::new(FxHashSet::default()))
        .lock();
    match lock.entry(hash(id)) {
        Entry::Occupied(_) => None,
        Entry::Vacant(entry) => {
            entry.insert();
            Some(f())
        }
    }
}

#[inline(always)]
pub fn clear_once_cache() {
    COMPLETION_SET.get().inspect(|s| s.lock().clear());
}

#[inline(always)]
#[track_caller]
/// Executes the function only the first time it is invoked from a given source location.
/// Similar to `once`, but uses the caller’s source location as the unique identifier.
pub fn once_at_source<T>(f: impl FnOnce() -> T) -> Option<T> {
    let location = std::panic::Location::caller();
    once(location, f)
}

#[macro_export]
macro_rules! assert_once {
    () => {
        $crate::assert_once!("Repeated call to assert_once")
    };
    ($($arg:tt)+) => {
        assert!(
            $crate::once_at_source(|| ()).is_some(),
            $($arg)*
        );
    }
}
