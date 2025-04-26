pub trait FluentBuilder {
    fn map<U>(self, f: impl FnOnce(Self) -> U) -> U
    where
        Self: Sized,
    {
        f(self)
    }

    fn when(self, condition: bool, then: impl FnOnce(Self) -> Self) -> Self
    where
        Self: Sized,
    {
        self.map(|this| if condition { then(this) } else { this })
    }

    fn when_or_else<R>(
        self,
        condition: bool,
        then: impl FnOnce(Self) -> R,
        or_else: impl FnOnce(Self) -> R,
    ) -> R
    where
        Self: Sized,
    {
        if condition { then(self) } else { or_else(self) }
    }

    fn when_some<T>(self, option: Option<T>, then: impl FnOnce(Self, T) -> Self) -> Self
    where
        Self: Sized,
    {
        self.map(|this| {
            if let Some(value) = option {
                then(this, value)
            } else {
                this
            }
        })
    }

    fn when_none<T>(self, option: Option<T>, then: impl FnOnce(Self) -> Self) -> Self
    where
        Self: Sized,
    {
        self.map(|this| if option.is_none() { then(this) } else { this })
    }

    fn when_some_or_else<T, R>(
        self,
        option: Option<T>,
        then: impl FnOnce(Self, T) -> R,
        or_else: impl FnOnce(Self) -> R,
    ) -> R
    where
        Self: Sized,
    {
        if let Some(value) = option {
            then(self, value)
        } else {
            or_else(self)
        }
    }
}
