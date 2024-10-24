/// Validates a value with a given validator.
#[macro_export]
macro_rules! validate {
    ($value:expr, $validator:expr) => {{
        use $crate::PropertyValidator as _;
        let validator = $validator;
        match validator.is_valid(&$value) {
            true => Ok(()),
            false => Err(endors::ValidationFailure::new(
                validator.message(),
                stringify!($value).to_string(),
            )),
        }
    }};
    ($value:expr, $validator:expr => $message:expr) => {{
        use $crate::PropertyValidator as _;
        let validator = $validator;
        match validator.is_valid(&$value) {
            true => Ok(()),
            false => Err(endors::ValidationFailure::new(
                $message.to_string(),
                stringify!($value).to_string(),
            )),
        }
    }};
}

/// Collects all error results into a single error.
#[macro_export]
macro_rules! collect_results {
    ($($validation:expr),+ $(,)?) => {{
        let mut errors = Vec::new();
        $(
            if let Err(err) = $validation {
                errors.push(err);
            }
        )+
        if errors.is_empty() {
            Ok(())
        } else {
            Err(endors::Error::new(errors))
        }
    }};
}
