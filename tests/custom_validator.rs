use endors::{
    collect_results, validate,
    validators::{Len, LessThan, NotEqual},
    Validator,
};

struct DummyModel {
    a: String,
    b: i32,
    c: bool,
}

impl DummyModel {
    fn calculated(&self) -> i32 {
        self.b + self.c as i32
    }
}

struct DummyValidator;
impl Validator<&DummyModel> for DummyValidator {
    fn validate(&self, value: &DummyModel) -> Result<(), endors::Error> {
        collect_results!(
            validate!(value.a, Len { min: 1, max: 100 }),
            validate!(value.a, |a: &String| a.starts_with("abc") => "a must start with abc"),
            validate!(value.b, LessThan(5)),
            validate!(value.c, NotEqual(true)),
            validate!(value.calculated(), NotEqual(6)),
        )
    }
}

#[test]
fn no_failures_when_valid_model() {
    let val = DummyModel {
        a: "abc".to_string(),
        b: 4,
        c: false,
    };
    let result = DummyValidator.validate(&val);
    assert!(result.is_ok())
}

#[test]
fn two_failures_when_a_is_empty() {
    let val = DummyModel {
        a: "".to_string(),
        b: 4,
        c: false,
    };
    let result = DummyValidator.validate(&val);
    assert!(result.is_err());
    assert_eq!(2, result.unwrap_err().failures().len())
}

#[test]
fn single_failure_when_a_does_not_start_with_abc() {
    let val = DummyModel {
        a: "test".to_string(),
        b: 4,
        c: false,
    };
    let result = DummyValidator.validate(&val);
    assert!(result.is_err());
    assert_eq!(1, result.unwrap_err().failures().len())
}

#[test]
fn single_failure_when_b_greater_than_four() {
    let val = DummyModel {
        a: "abc".to_string(),
        b: 5,
        c: false,
    };
    let result = DummyValidator.validate(&val);
    assert!(result.is_err());
    assert_eq!(1, result.unwrap_err().failures().len())
}

#[test]
fn single_failure_when_c_equal_to_true() {
    let val = DummyModel {
        a: "abc".to_string(),
        b: 4,
        c: true,
    };
    let result = DummyValidator.validate(&val);
    assert!(result.is_err());
    assert_eq!(1, result.unwrap_err().failures().len())
}

#[test]
fn four_failures_when_all_fields_invalid() {
    let val = DummyModel {
        a: "".to_string(),
        b: 5,
        c: true,
    };
    let result = DummyValidator.validate(&val);
    assert!(result.is_err());
    assert_eq!(5, result.unwrap_err().failures().len())
}
