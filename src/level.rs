use crate::layer::Layer;

pub struct Level {
    inner: ldtk_deser_json::project::level::Level,
}
impl Level {
    pub fn new(inner: ldtk_deser_json::project::level::Level) -> Self {
        Self { inner }
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
}
