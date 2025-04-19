use std::ops::{Add, Div, Mul, Range, RangeInclusive, Sub};

// -----------------------------------------------------------------------------

pub trait RangeExt<T>: Sized {
    fn from_exclusive(start: T, end: T) -> Self;
    fn start(&self) -> &T;
    fn end(&self) -> &T;

    fn intersect(&self, other: &Self) -> Self
    where
        T: Clone + Ord,
    {
        Self::from_exclusive(
            std::cmp::max(self.start().clone(), other.start().clone()),
            std::cmp::min(self.end().clone(), other.end().clone()),
        )
    }

    fn map<U>(self, f: impl Fn(T) -> U) -> Range<U>
    where
        T: Clone,
    {
        f(self.start().clone())..f(self.end().clone())
    }
}

impl<T: Ord + Clone> RangeExt<T> for std::ops::Range<T> {
    fn from_exclusive(start: T, end: T) -> Self {
        start..end
    }

    fn start(&self) -> &T {
        &self.start
    }

    fn end(&self) -> &T {
        &self.end
    }
}

// -----------------------------------------------------------------------------

pub trait RangeInclusiveExt<T>: Sized {
    fn from_inclusive(start: T, end: T) -> Self;
    fn start(&self) -> &T;
    fn end(&self) -> &T;

    fn map<U>(self, f: impl Fn(T) -> U) -> RangeInclusive<U>
    where
        T: Clone,
    {
        f(self.start().clone())..=f(self.end().clone())
    }

    #[inline(always)]
    fn intersect(&self, other: &Self) -> Self
    where
        T: Clone + Ord,
    {
        Self::from_inclusive(
            std::cmp::max(self.start().clone(), other.start().clone()),
            std::cmp::min(self.end().clone(), other.end().clone()),
        )
    }

    #[inline(always)]
    fn lerp<F>(self, fraction: F) -> T
    where
        T: Add<Output = T> + Copy,
        F: From<u8> + Sub<Output = F> + Mul<T, Output = T> + Copy,
    {
        (F::from(1u8) - fraction) * *self.start() + fraction * *self.end()
    }

    #[inline(always)]
    fn fraction(self, value: T) -> f32
    where
        T: Sub<Output = T> + Div<Output = f32> + Copy,
    {
        (value - *self.start()) / (*self.end() - *self.start())
    }

    #[inline(always)]
    fn fraction_clamp(self, value: T) -> f32
    where
        T: Sub<Output = T> + Div<Output = f32> + Copy,
    {
        self.fraction(value).clamp(0f32, 1f32)
    }

    #[inline(always)]
    /// Linearly remap a value from one range to another,
    /// so that when `x == self.start()` returns `to.start()`
    /// and when `x == self.end()` returns `to.end()`.
    fn remap(self, value: T, to: impl Into<RangeInclusive<T>>) -> T
    where
        T: Copy
            + PartialEq
            + Sub<Output = T>
            + Div<Output = T>
            + Add<Output = T>
            + Mul<Output = T>
            + From<u8>,
    {
        let from = self;
        let to = to.into();
        debug_assert!(from.start() != from.end());
        let t = (value - *from.start()) / (*from.end() - *from.start());
        to.lerp(t)
    }

    #[inline(always)]
    /// Like [`remap`], but also clamps the value so that the returned value is always in the `to` range.
    fn remap_clamp(self, value: T, to: impl Into<RangeInclusive<T>>) -> T
    where
        T: Copy
            + PartialEq
            + From<u8>
            + PartialOrd
            + Sub<Output = T>
            + Add<Output = T>
            + Mul<Output = T>
            + Div<Output = T>,
    {
        let from = self;
        let to = to.into();
        if from.end() < from.start() {
            return Self::from_inclusive(*from.end(), *from.start())
                .remap_clamp(value, *to.end()..=*to.start());
        }
        if value <= *from.start() {
            *to.start()
        } else if *from.end() <= value {
            *to.end()
        } else {
            debug_assert!(from.start() != from.end());
            let t = (value - *from.start()) / (*from.end() - *from.start());
            // Ensure no numerical inaccuracies sneak in:
            if T::from(1u8) <= t {
                *to.end()
            } else {
                to.lerp(t)
            }
        }
    }
}

impl<T> RangeInclusiveExt<T> for RangeInclusive<T> {
    fn from_inclusive(start: T, end: T) -> Self {
        start..=end
    }

    fn start(&self) -> &T {
        self.start()
    }

    fn end(&self) -> &T {
        self.end()
    }
}
