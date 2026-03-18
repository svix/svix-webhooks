use std::panic::Location;

/// can_fail() hides errors from execution, but still logs them
/// as errors for investigation.
///
/// This is useful if an error can't meaningfully be propagated.
pub trait CanFail {
    type Output;
    fn can_fail(self, detail: &'static str) -> Self::Output;
    fn warn_on_fail(self, detail: &'static str) -> Self::Output;
}

impl<T: Default, E: std::fmt::Debug> CanFail for Result<T, E> {
    type Output = T;

    #[track_caller]
    fn can_fail(self, detail: &'static str) -> Self::Output {
        let l = Location::caller();
        self.unwrap_or_else(|e| {
            let (file, line) = (l.file(), l.line());
            tracing::error!(error = ?e, "swallowing error at {file}:{line} {detail}");
            T::default()
        })
    }

    #[track_caller]
    fn warn_on_fail(self, detail: &'static str) -> Self::Output {
        let l = Location::caller();
        self.unwrap_or_else(|e| {
            let (file, line) = (l.file(), l.line());
            tracing::warn!(error = ?e, "swallowing error at {file}:{line} {detail}");
            T::default()
        })
    }
}
