use crate::gen::SchemaGenerator;
use crate::schema::*;
use crate::JsonSchema;
use ndarray::ArrayBase;
use ndarray::Dimension;
use ndarray::Data;
use ndarray::Dim;
use ndarray::Ix;
use ndarray::IxDyn;


impl <T> JsonSchema for Dim<T> where T: JsonSchema{
    fn schema_name() -> String {
        format!("Dimension_of_type_{}", <T>::schema_name())
    }

    fn json_schema(gen: &mut crate::gen::SchemaGenerator) -> Schema {
        <T>::json_schema(gen)
    }
}

impl JsonSchema for IxDyn {
    fn schema_name() -> String {
        "DynamicArrayDimension".to_owned()
    }

    fn json_schema(gen: &mut crate::gen::SchemaGenerator) -> Schema {
        Vec::<Ix>::json_schema(gen)
    }
}

impl<T, D, S> JsonSchema for ArrayBase<S, D>
where T: JsonSchema, D: Dimension + JsonSchema, S:Data<Elem=T> {
    fn schema_name() -> String {
        format!("Array_of_type_{}_{}",T::schema_name(), D::schema_name())
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        let mut schema = SchemaObject {
            instance_type: Some(InstanceType::Object.into()),
            ..Default::default()
        };
        let obj = schema.object();
        obj.required.insert("v".to_owned());
        obj.required.insert("dim".to_owned());
        obj.required.insert("data".to_owned());
        obj.properties
            .insert("v".to_owned(), <u8>::json_schema(gen));
        obj.properties
            .insert("dim".to_owned(), D::json_schema(gen));
        obj.properties
            .insert("data".to_owned(), <Vec<T>>::json_schema(gen));
        schema.into()
    }
}