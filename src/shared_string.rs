use smol_str::SmolStr;

use serde::{Deserialize, Serialize};
use std::{
    borrow::{Borrow, Cow},
    ops::Deref,
    sync::Arc,
};

// -----------------------------------------------------------------------------

/// A shared string is an immutable string that can be cheaply cloned and can be stack allocated if it is smaller than 24 bytes.
/// For now, it is just a wrapper around [`SmolStr`].
#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Clone)]
pub struct SharedString(SmolStr);

impl Deref for SharedString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

impl SharedString {
    /// Creates a static [`SharedString`] from a `&'static str`.
    pub const fn new_static(str: &'static str) -> Self {
        Self(SmolStr::new_static(str))
    }
}

impl Default for SharedString {
    fn default() -> Self {
        Self::new_static("")
    }
}

impl AsRef<str> for SharedString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for SharedString {
    fn borrow(&self) -> &str {
        self.as_ref()
    }
}

impl std::fmt::Debug for SharedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Display for SharedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

impl PartialEq<String> for SharedString {
    fn eq(&self, other: &String) -> bool {
        self.as_ref() == other
    }
}

impl PartialEq<SharedString> for String {
    fn eq(&self, other: &SharedString) -> bool {
        self == other.as_ref()
    }
}

impl PartialEq<str> for SharedString {
    fn eq(&self, other: &str) -> bool {
        self.as_ref() == other
    }
}

impl<'a> PartialEq<&'a str> for SharedString {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}

impl From<&str> for SharedString {
    #[inline]
    fn from(value: &str) -> Self {
        SharedString(SmolStr::from(value))
    }
}

impl From<&mut str> for SharedString {
    #[inline]
    fn from(value: &mut str) -> Self {
        SharedString(SmolStr::from(value))
    }
}

impl From<String> for SharedString {
    #[inline]
    fn from(value: String) -> Self {
        SharedString(SmolStr::from(value))
    }
}

impl From<&String> for SharedString {
    #[inline]
    fn from(value: &String) -> Self {
        SharedString(SmolStr::from(value))
    }
}

impl From<Box<str>> for SharedString {
    #[inline]
    fn from(value: Box<str>) -> Self {
        SharedString(SmolStr::from(value))
    }
}

impl From<&Box<str>> for SharedString {
    #[inline]
    fn from(value: &Box<str>) -> Self {
        SharedString(SmolStr::from(value.as_ref()))
    }
}

impl From<Arc<str>> for SharedString {
    #[inline]
    fn from(value: Arc<str>) -> Self {
        SharedString(SmolStr::from(value.as_ref()))
    }
}

impl From<&Arc<str>> for SharedString {
    #[inline]
    fn from(value: &Arc<str>) -> Self {
        SharedString(SmolStr::from(value.as_ref()))
    }
}

impl<'a> From<Cow<'a, str>> for SharedString {
    #[inline]
    fn from(value: Cow<'a, str>) -> Self {
        SharedString(SmolStr::from(value.as_ref()))
    }
}

impl From<SharedString> for String {
    #[inline]
    fn from(val: SharedString) -> Self {
        val.0.to_string()
    }
}

impl Serialize for SharedString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

impl<'de> Deserialize<'de> for SharedString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(SharedString::from(s))
    }
}
