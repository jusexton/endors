use thiserror::Error;
pub mod macros;
pub mod validators;

pub trait Validator<T: ?Sized> {
    fn validate(&self, value: T) -> Result<(), crate::Error>;
}

/// Validator meant for primitive and generally simple value validation.
pub trait PropertyValidator<T: ?Sized> {
    fn is_valid(&self, value: &T) -> bool;

    fn message(&self) -> String;
}

impl<T, F> PropertyValidator<T> for F
where
    F: Fn(&T) -> bool,
{
    fn is_valid(&self, value: &T) -> bool {
        self(value)
    }

    fn message(&self) -> String {
        todo!()
    }
}

#[derive(Debug)]
pub struct ValidationFailure {
    message: String,
    property_path: String,
}

impl std::fmt::Display for ValidationFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} => {}", self.property_path, self.message)
    }
}

impl ValidationFailure {
    pub fn new(message: String, property_path: String) -> Self {
        Self {
            message,
            property_path,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn property_path(&self) -> &str {
        &self.property_path
    }
}

#[derive(Error, Debug)]
#[error("Validation failed:\n{}", .failures.iter().map(|f| f.to_string()).collect::<Vec<_>>().join("\n"))]
pub struct Error {
    failures: Vec<ValidationFailure>,
}

impl From<ValidationFailure> for Error {
    fn from(value: ValidationFailure) -> Self {
        Self {
            failures: vec![value],
        }
    }
}

impl Error {
    pub fn new(failures: Vec<ValidationFailure>) -> Self {
        Self { failures }
    }

    pub fn failures(&self) -> &Vec<ValidationFailure> {
        &self.failures
    }
}
