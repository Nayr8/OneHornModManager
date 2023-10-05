
pub trait ResultExtensions {
    type Success;
    type Error;

    fn on_ok(self, op: impl FnOnce(&Self::Success)) -> Self;
    fn on_err(self, op: impl FnOnce(&Self::Error)) -> Self;
}

impl<T, U> ResultExtensions for Result<T, U> {
    type Success = T;
    type Error = U;

    fn on_ok(self, op: impl FnOnce(&Self::Success)) -> Self {
        if let Ok(s) = &self {
            op(s)
        }
        self
    }

    fn on_err(self, op: impl FnOnce(&Self::Error)) -> Self {
        if let Err(e) = &self {
            op(e)
        }
        self
    }
}

pub trait OptionExtensions {
    type Type;

    fn on_some(self, op: impl FnOnce(&Self::Type)) -> Self;
    fn on_none(self, op: impl FnOnce()) -> Self;
}

impl<T> OptionExtensions for Option<T> {
    type Type = T;

    fn on_some(self, op: impl FnOnce(&Self::Type)) -> Self {
        if let Some(t) = &self {
            op(t)
        }
        self
    }

    fn on_none(self, op: impl FnOnce()) -> Self {
        if let None = &self {
            op()
        }
        self
    }
}