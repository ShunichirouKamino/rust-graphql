use std::f64::NAN;

use crate::domain::my_float::MyFloat;

#[test]
fn test_object_eq_ok() {
    let num: f64 = 1.111;
    let result_1 = MyFloat::of(num);
    let result_2 = MyFloat::of(num);
    assert_eq!(result_1, result_2);
    assert_eq!(f64::from(result_1), f64::from(result_2));
}

#[test]
fn test_nan_ne_ok() {
    let num: f64 = NAN;
    let result_1 = MyFloat::of(num);
    let result_2 = MyFloat::of(num);
    assert_ne!(result_1, result_2);
    assert_ne!(f64::from(result_1), f64::from(result_2));
}
