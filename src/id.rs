#[macro_export]
macro_rules! id_type {
    (struct $name: ident) => {
        id_type!(@internal $name,);
    };
    (pub struct $name: ident) => {
        id_type!(@internal $name, pub);
    };
    (pub ( $($vis:tt)+ ) struct $name: ident) => {
        id_type!(@internal $name, pub ($($vis)*));
    };
    (@internal $name: ident, $($vis:tt)*) => {
        #[derive(
            Debug, Default, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
        )]
        $($vis)* struct $name(u64);

        impl $name {
            #[inline(always)]
            pub fn first() -> Self {
                Self(0)
            }

            #[inline(always)]
            pub fn next(self) -> Self {
                Self(self.0 + 1)
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
    (struct $name: ident) => {
        global_id_type!(@internal $name,);
    };
    (pub struct $name: ident) => {
        global_id_type!(@internal $name, pub);
    };
    (pub ( $($vis:tt)+ ) struct $name: ident) => {
        global_id_type!(@internal $name, pub ($($vis)*));
    };
    (@internal $name: ident, $($vis:tt)*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        $($vis)* struct $name(u64);

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
