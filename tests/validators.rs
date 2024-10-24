use endors::{
    validators::{Equal, InRange, IsSome, NotEqual},
    PropertyValidator,
};

#[test]
fn valid_when_some() {
    let result = IsSome.is_valid(&Some(()));
    assert!(result)
}

#[test]
fn invalid_when_none() {
    let result = IsSome.is_valid(&None::<()>);
    assert!(!result)
}

#[test]
fn valid_when_in_range() {
    let result = InRange { min: 0, max: 10 }.is_valid(&5);
    assert!(result)
}

#[test]
fn invalid_when_out_of_range() {
    let result = InRange { min: 0, max: 10 }.is_valid(&11);
    assert!(!result)
}

#[test]
fn valid_when_equal() {
    let result = Equal(10).is_valid(&10);
    assert!(result)
}

#[test]
fn invalid_when_not_equal() {
    let result = Equal(10).is_valid(&9);
    assert!(!result)
}

#[test]
fn valid_when_not_equal() {
    let result = NotEqual(10).is_valid(&9);
    assert!(result)
}

#[test]
fn invalid_when_equal() {
    let result = NotEqual(10).is_valid(&10);
    assert!(!result)
}
