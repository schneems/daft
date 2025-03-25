use std::collections::VecDeque;

/// Accumulate and combine errors
///
/// Assuming `T` is necessary for code generation:
/// - Errors that block code generation should return `Result<T, syn::Error>`
/// - Errors that do not block code generation should return `(T, Option<syn::Error>)`
/// - Errors that may or may not block code generation should return `Result<T, Option<syn::Error>, syn::Error>`.
///   `Ok(T, None)` indicates no errors
///   `Ok(T, Some())` indicates an error that did not block code generation.
///   `Err()` indicates code could not generate due to error
#[derive(Debug, Default)]
pub(crate) struct ErrorStore {
    errors: VecDeque<syn::Error>,
}

impl ErrorStore {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn push(&mut self, error: syn::Error) {
        self.errors.push_back(error);
    }

    pub(crate) fn first_to_syn(self) -> Option<syn::Error> {
        let mut errors = self.errors;
        if let Some(mut error) = errors.pop_front() {
            for e in errors {
                error.combine(e);
            }
            Some(error)
        } else {
            None
        }
    }
}
