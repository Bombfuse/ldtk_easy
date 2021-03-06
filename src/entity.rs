use std::collections::HashMap;

#[derive(Clone)]
pub struct Entity {
    inner: ldtk_deser_json::project::instances::entity_instance::EntityInstance,
}
impl Entity {
    pub fn new(
        inner: ldtk_deser_json::project::instances::entity_instance::EntityInstance,
    ) -> Self {
        Self { inner }
    }

    pub fn identifier(&self) -> String {
        self.inner.identifier.clone()
    }

    pub fn uid(&self) -> String {
        self.inner.iid.clone()
    }

    pub fn pivot(&self) -> (f32, f32) {
        (self.inner.pivot[0], self.inner.pivot[1])
    }

    pub fn width(&self) -> i32 {
        self.inner.width
    }

    pub fn height(&self) -> i32 {
        self.inner.height
    }

    pub fn grid_coords(&self) -> (i32, i32) {
        (self.inner.grid_coords[0], self.inner.grid_coords[1])
    }

    pub fn pixel_coordinates(&self) -> (i32, i32) {
        (self.inner.px[0], self.inner.px[1])
    }

    pub fn fields(&self) -> HashMap<String, Field> {
        self.inner
            .field_instances
            .iter()
            .map(|field_instance| {
                let field = parse_field(&field_instance.value);
                (field_instance.identifier.clone(), field)
            })
            .collect()
    }

    pub fn field<T: Into<String>>(&self, field_name: T) -> Option<Field> {
        let field_name: String = field_name.into();
        self.inner
            .field_instances
            .iter()
            .find(|field| field.identifier == field_name)
            .map(|field| parse_field(&field.value))
    }

    pub fn tags(&self) -> Vec<String> {
        self.inner.tags.clone()
    }
}

pub fn parse_field(value: &ldtk_deser_json::serde_json::Value) -> Field {
    if value.is_null() {
        return Field::Null;
    }

    if value.is_boolean() {
        if let Some(value) = value.as_bool() {
            return Field::Bool { value };
        }
    }

    if value.is_f64() {
        if let Some(value) = value.as_f64() {
            return Field::Float { value };
        }
    }

    if value.is_i64() {
        if let Some(value) = value.as_i64() {
            return Field::Int { value };
        }
    }

    if value.is_string() {
        if let Some(value) = value.as_str() {
            return Field::String {
                value: value.to_string(),
            };
        }
    }

    if value.is_object() {
        if let Some(value_object) = value.as_object() {
            let mut map = HashMap::new();

            for key in value_object.keys() {
                if let Some(property_value) = value_object.get(key) {
                    map.insert(key.clone(), parse_field(property_value));
                }
            }

            return Field::Map { value: map };
        }
    }

    if value.is_array() {
        if let Some(array) = value.as_array() {
            let mut value = Vec::new();

            for array_value in array {
                value.push(parse_field(array_value));
            }

            return Field::Array { value };
        }
    }

    // Failed to parse, return null
    Field::Null
}

#[derive(Clone, PartialEq)]
pub enum Field {
    Null,
    Array { value: Vec<Field> },
    Float { value: f64 },
    Int { value: i64 },
    Bool { value: bool },
    String { value: String },
    Map { value: HashMap<String, Field> },
}
