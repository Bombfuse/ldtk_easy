use crate::{
    entity::{parse_field, Field},
    layer::Layer,
};

#[derive(Clone)]
pub struct Level {
    inner: ldtk_deser_json::project::level::Level,
}
impl Level {
    pub fn new(inner: ldtk_deser_json::project::level::Level) -> Self {
        Self { inner }
    }

    pub fn identifier(&self) -> String {
        self.inner.identifier.clone()
    }

    pub fn get_layer<T: Into<String>>(&self, layer_name: T) -> Option<Layer> {
        let layer_name: String = layer_name.into();

        if let Some(layer_instances) = &self.inner.layer_instances {
            let layer = layer_instances.iter().find(|l| l.identifier == layer_name);

            if let Some(layer) = layer {
                return Some(Layer::new(layer.clone()));
            }
        }

        None
    }

    pub fn field<T: Into<String>>(&self, field_name: T) -> Option<Field> {
        let field_name: String = field_name.into();
        if let Some(field) = self
            .inner
            .field_instances
            .iter()
            .find(|field| field.identifier == field_name)
        {
            return Some(parse_field(&field.value));
        }

        None
    }

    pub fn pixel_size(&self) -> (usize, usize) {
        (self.inner.px_wid as usize, self.inner.px_hei as usize)
    }
}
