# Endors

Warning: Endors is not production ready and currently has a very unstable API. Use are your own risk.

Validation framework written in the rust programming language.

## Quick Look

```rs
struct User {
    first_name: String,
    last_name: String,
    phone_number: Option<String>
}

struct UserValidator;
impl Validator<&User> for UserValidator {
    fn validate(&self, value: &User) -> Result<(), endors::Error> {
        // Perform many validations and collect all the results into a single result.
        collect_results!(
            validate!(value.first_name, Len { min: 1, max: 100 }), // Uses default len error message.
            validate!(
                value.first_name, 
                |s: &str| s.len() % 2 == 0 => "First name must have an even number of characters." // Custom error message
            ),
            validate!(
                value.last_name, 
                NotEqual(value.first_name) => "Last name must not equal first name."
            ),
            validate!(
                value.phone_number, 
                IsSome => "Phone number must be provided."
            ),
        )

        // OR

        // Question mark operator to fail fast and only return the first error that occurs.
        validate!(value.first_name, Len { min: 1, max: 100 })?;
        validate!(value.first_name, |s: &str| s.len() % 2 == 0 => "First name must have an even number of characters.")?;
        validate!(value.last_name, NotEqual(value.first_name) => "Last name must not equal first name.")?;
        validate!(value.phone_number, IsSome => "Phone number must be provided."?;
    }
}

let result = UserValidator.validate(&User { 
    first_name: "John".to_string(), 
    last_name: "Doe".to_string(),
    phone_number: Some("123-456-7890".to_string())
});
assert!(result.is_ok())
```

