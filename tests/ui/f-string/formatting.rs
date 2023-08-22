// run-pass
#![feature(f_strings)]

pub fn main() {
    let value = 1.2345;
    assert_eq!(f"value = {value:.2}", "value = 1.235");
}
