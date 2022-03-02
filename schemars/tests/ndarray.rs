mod util;
use ndarray::{Array0, Array1, Array2, Array3, Array4};
use schemars::{schema_for, JsonSchema};
use jsonschema::{Draft, JSONSchema};
use util::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
struct TestStruct{
    zero: Array0<f32>,
    one: Array1<f64>,
    two: Array2<u32>,
    three: Array3<u64>,
    four: Array4<i32>
}


#[test]
fn ndarray() -> TestResult {
    test_default_generated_schema::<TestStruct>("ndarray")
}

#[test]
fn array_serialisation() {
    let test_schema = schema_for!(TestStruct);
    let schema = serde_json::to_string(&test_schema).unwrap();
    let schema_value: serde_json::Value = serde_json::from_str(&schema).unwrap();
    let test_struct = TestStruct{zero: Array0::zeros(()), one: Array1::zeros(2), two: Array2::zeros((3,4)), three: Array3::zeros((1,3,4)), four: Array4::zeros((2,1,2,2))};
    let test_json = serde_json::to_string(&test_struct).expect("serialisation of test struct failed");
    let test_value: serde_json::Value = serde_json::from_str(&test_json).unwrap();
    let compiled_schema = JSONSchema::options().with_draft(Draft::Draft7).compile(&schema_value).expect("reading test_schema failed");
    let validation_result = compiled_schema.validate(&test_value);
    assert!(validation_result.is_ok());
}