use crate::entity::Entity;

#[derive(Clone)]
pub struct Layer {
    inner: ldtk_deser_json::project::instances::layer_instance::LayerInstance,
}
impl Layer {
    pub fn new(inner: ldtk_deser_json::project::instances::layer_instance::LayerInstance) -> Self {
        Self { inner }
    }

    pub fn entities(&self) -> Vec<Entity> {
        self.inner
            .entity_instances
            .iter()
            .map(|instance| Entity::new(instance.clone()))
            .collect()
    }
}
