use ldtk_deser_json::project::instances::tile_instance::TileInstance;

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

    pub fn relative_tileset_path(&self) -> Option<String> {
        self.inner.tileset_rel_path.clone()
    }

    pub fn tile_size(&self) -> i32 {
        self.inner.grid_size
    }

    pub fn int_grid(&self) -> Vec<i32> {
        self.inner.int_grid_csv.clone()
    }

    pub fn width(&self) -> i32 {
        self.inner.c_wid
    }

    pub fn height(&self) -> i32 {
        self.inner.c_hei
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width() as usize, self.height() as usize)
    }

    pub fn pixel_size(&self) -> (usize, usize) {
        (
            (self.inner.c_wid * self.inner.grid_size) as usize,
            (self.inner.c_hei * self.inner.grid_size) as usize,
        )
    }

    pub fn autotiles(&self) -> Vec<Autotile> {
        self.inner
            .auto_layer_tiles
            .clone()
            .into_iter()
            .map(|instance| Autotile {
                source: (instance.src[0], instance.src[1]),
                pixel_position: (instance.px[0], instance.px[1]),
                flip: instance.f,
            })
            .collect()
    }
}

pub struct Autotile {
    pub source: (i32, i32),
    pub pixel_position: (i32, i32),
    pub flip: i32,
}
