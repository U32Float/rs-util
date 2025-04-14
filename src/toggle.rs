use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------

/// An Option-like type that remembers the tagged value.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Toggle<T> {
    On(T),
    Off(T),
}

impl<T: Default> Default for Toggle<T> {
    fn default() -> Self {
        Self::Off(T::default())
    }
}

impl<T> Toggle<T> {
    pub fn apply_option(self, option: Option<T>) -> Self {
        match option {
            Some(val) => Self::On(val),
            None => self.toggle_off(),
        }
    }

    pub fn is_on(&self) -> bool {
        matches!(self, Toggle::On(_))
    }

    pub fn is_off(&self) -> bool {
        matches!(self, Toggle::Off(_))
    }

    pub fn unwrap(self) -> T {
        match self {
            Toggle::On(inner) => inner,
            Toggle::Off(inner) => inner,
        }
    }

    pub fn unwrap_ref(&self) -> &T {
        match self {
            Toggle::On(inner) => inner,
            Toggle::Off(inner) => inner,
        }
    }

    pub fn unwrap_mut(&mut self) -> &mut T {
        match self {
            Toggle::On(inner) => inner,
            Toggle::Off(inner) => inner,
        }
    }

    pub fn unwrap_on_or(self, default: T) -> T {
        match self {
            Toggle::On(inner) => inner,
            Toggle::Off(_) => default,
        }
    }

    pub fn unwrap_off_or(self, default: T) -> T {
        match self {
            Toggle::On(_) => default,
            Toggle::Off(inner) => inner,
        }
    }

    pub fn toggle(self) -> Self {
        match self {
            Toggle::On(inner) => Self::Off(inner),
            Toggle::Off(inner) => Self::On(inner),
        }
    }

    pub fn toggle_on(self) -> Self {
        match self {
            Toggle::On(inner) => Self::On(inner),
            Toggle::Off(inner) => Self::On(inner),
        }
    }

    pub fn toggle_off(self) -> Self {
        match self {
            Toggle::On(inner) => Self::Off(inner),
            Toggle::Off(inner) => Self::Off(inner),
        }
    }

    pub fn map<U>(self, map: impl FnOnce(T) -> U) -> Toggle<U> {
        match self {
            Toggle::On(inner) => Toggle::On(map(inner)),
            Toggle::Off(inner) => Toggle::Off(map(inner)),
        }
    }

    pub fn as_ref(&self) -> Toggle<&T> {
        match self {
            Toggle::On(inner) => Toggle::On(inner),
            Toggle::Off(inner) => Toggle::Off(inner),
        }
    }

    pub fn as_mut(&mut self) -> Toggle<&mut T> {
        match self {
            Toggle::On(inner) => Toggle::On(inner),
            Toggle::Off(inner) => Toggle::Off(inner),
        }
    }
}

impl<T> From<Toggle<T>> for Option<T> {
    fn from(value: Toggle<T>) -> Self {
        match value {
            Toggle::On(inner) => Some(inner),
            Toggle::Off(_) => None,
        }
    }
}
