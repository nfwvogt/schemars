use crate::gen::SchemaGenerator;
use crate::schema::*;
use crate::JsonSchema;
use num_complex::Complex;


impl<T> JsonSchema for Complex<T>
where
    T: JsonSchema,
{
    no_ref_schema!();

    fn schema_name() -> String {
        format!(
            "Complex_for_type_{}",
            T::schema_name()
        )
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        // num_complex 0.4 serializes a Complex number 
        // as a tuple of real and imaginary parts
        <(T, T)>::json_schema(gen)
    }
}
