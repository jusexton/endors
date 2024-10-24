use crate::PropertyValidator;

/// Validate that an optional value contains a value.
pub struct IsSome;
impl<T> PropertyValidator<Option<T>> for IsSome {
    fn is_valid(&self, value: &Option<T>) -> bool {
        value.is_some()
    }

    fn message(&self) -> String {
        "Expected value to exist".to_string()
    }
}

/// Validate that an optional value does not contain a value.
pub struct IsNone;
impl<T> PropertyValidator<Option<T>> for IsNone {
    fn is_valid(&self, value: &Option<T>) -> bool {
        value.is_none()
    }

    fn message(&self) -> String {
        "Expected value to not exist".to_string()
    }
}

/// Validate that a value falls within a given range.
pub struct InRange<T>
where
    T: PartialOrd,
{
    pub min: T,
    pub max: T,
}

impl<T> PropertyValidator<T> for InRange<T>
where
    T: PartialOrd,
{
    fn is_valid(&self, value: &T) -> bool {
        self.min <= *value && self.max >= *value
    }

    fn message(&self) -> String {
        "Expected value to be in range".to_string()
    }
}

/// Validate that a value equals another.
pub struct Equal<T>(pub T)
where
    T: PartialEq;

impl<T> PropertyValidator<T> for Equal<T>
where
    T: PartialEq,
{
    fn is_valid(&self, value: &T) -> bool {
        *value == self.0
    }

    fn message(&self) -> String {
        "Expected values to be equal".to_string()
    }
}

/// Validate that a value does not equal another.
pub struct NotEqual<T>(pub T)
where
    T: PartialEq;

impl<T> PropertyValidator<T> for NotEqual<T>
where
    T: PartialEq,
{
    fn is_valid(&self, value: &T) -> bool {
        *value != self.0
    }

    fn message(&self) -> String {
        "Expected value to not be equal".to_string()
    }
}

/// Validate that a values len falls within a given range.
pub struct Len {
    pub min: usize,
    pub max: usize,
}

impl PropertyValidator<String> for Len {
    fn is_valid(&self, value: &String) -> bool {
        InRange {
            min: self.min,
            max: self.max,
        }
        .is_valid(&value.len())
    }

    fn message(&self) -> String {
        "Expected value length to be in range".to_string()
    }
}

/// Validate that a values len is no less than a given minimum.
pub struct MinLen(pub usize);
impl PropertyValidator<String> for MinLen {
    fn is_valid(&self, value: &String) -> bool {
        value.len() >= self.0
    }

    fn message(&self) -> String {
        format!("Expected value length to be no less than {}", self.0)
    }
}

/// Validate that a values len is no less than a given maximum
pub struct MaxLen(pub usize);
impl PropertyValidator<String> for MaxLen {
    fn is_valid(&self, value: &String) -> bool {
        self.0 <= value.len()
    }

    fn message(&self) -> String {
        format!("Expected value length to be no more than {}", self.0)
    }
}

/// Validate that a value is less than a given value.
pub struct LessThan<T>(pub T)
where
    T: PartialOrd;

impl<T> PropertyValidator<T> for LessThan<T>
where
    T: PartialOrd,
{
    fn is_valid(&self, value: &T) -> bool {
        *value < self.0
    }

    fn message(&self) -> String {
        "Expected value to be lesser".to_string()
    }
}

/// Validate that a value is greater than a given value.
pub struct GreaterThan<T>(pub T)
where
    T: PartialOrd;

impl<T> PropertyValidator<T> for GreaterThan<T>
where
    T: PartialOrd,
{
    fn is_valid(&self, value: &T) -> bool {
        *value > self.0
    }

    fn message(&self) -> String {
        "Expected value to be greater".to_string()
    }
}

/// Validate that a value is less than or equal to a given value.
pub struct LessThanOrEqual<T>(pub T)
where
    T: PartialOrd;

/// Validate that a value is greater than or equal to a given value.
pub struct GreaterThanOrEqual<T>(pub T)
where
    T: PartialOrd;
