mod util;
use num_complex::Complex;
use util::*;


#[test]
fn num_complex_f64() -> TestResult {
    test_default_generated_schema::<Complex<f64>>("num_complex_f64")
}

#[test]
fn num_complex_i32() -> TestResult {
    test_default_generated_schema::<Complex<i32>>("num_complex_i32")
}