use std::num::NonZeroU64;

// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize, Debug)]
pub struct Id(NonZeroU64);

impl nohash_hasher::IsEnabled for Id {}

impl Id {
    pub const NULL: Self = Self(NonZeroU64::MAX);

    #[inline]
    const fn from_hash(hash: u64) -> Self {
        if let Some(nonzero) = NonZeroU64::new(hash) {
            Self(nonzero)
        } else {
            Self(NonZeroU64::MIN) // The hash was exactly zero (very bad luck)
        }
    }

    /// Generate a new [`Id`] by hashing some source (e.g. a string or integer).
    pub fn new(source: impl std::hash::Hash) -> Self {
        Self::from_hash(ahash::RandomState::with_seeds(1, 2, 3, 4).hash_one(source))
    }

    /// Generate a new [`Id`] by hashing the parent [`Id`] and the given argument.
    pub fn with(self, child: impl std::hash::Hash) -> Self {
        use std::hash::{BuildHasher, Hasher};
        let mut hasher = ahash::RandomState::with_seeds(1, 2, 3, 4).build_hasher();
        hasher.write_u64(self.0.get());
        child.hash(&mut hasher);
        Self::from_hash(hasher.finish())
    }

    pub fn value(&self) -> u64 {
        self.0.get()
    }
}

impl From<&str> for Id {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

// -----------------------------------------------------------------------------

#[macro_export]
macro_rules! id_type {
    ($($vis:vis struct $name:ident);* $(;)?)=> {
        $(id_type!(@internal $vis struct $name);)*
    };
    (@internal $vis:vis struct $name:ident) => {
        #[derive(
            Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
        )]
        $vis struct $name(u64);

        impl $name {
            #[inline(always)]
            pub fn first() -> Self {
                Self(0)
            }

            #[inline(always)]
            pub fn next(self) -> Self {
                Self(self.0 + 1)
            }

            #[inline(always)]
            pub fn post_inc(&mut self) -> Self {
                let current = *self;
                *self = self.next();
                current
            }
        }

        impl From<u64> for $name {
            fn from(value: u64) -> Self {
                Self(value)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "ID({})", self.0)
            }
        }
    };
}

#[macro_export]
macro_rules! global_id_type {
    ($($vis:vis struct $name:ident);* $(;)?)=> {
        $(global_id_type!(@internal $vis struct $name);)*
    };
    (@internal $vis:vis struct $name: ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
        $vis struct $name(u64);

        impl $name {
            #[inline(always)]
            pub fn next() -> Self {
                static NEXT_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
                Self(NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
            }
        }

        impl From<u64> for $name {
            fn from(value: u64) -> Self {
                Self(value)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "ID({})", self.0)
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::next()
            }
        }
    };
}
