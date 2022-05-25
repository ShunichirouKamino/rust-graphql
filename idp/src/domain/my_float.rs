#[derive(PartialEq, Clone, PartialOrd, Debug)]
pub struct MyFloat {
    num: f64,
}

impl MyFloat {
    pub fn of(num: f64) -> Self {
        MyFloat { num }
    }
}

impl From<MyFloat> for f64 {
    fn from(my_float: MyFloat) -> Self {
        my_float.num
    }
}
