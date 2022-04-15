use crate::{error::LDTKError, level::Level};

pub struct Project {
    inner: ldtk_deser_json::project::Project,
}
impl Project {
    pub fn new<T: Into<String>>(json: T) -> Result<Self, LDTKError> {
        match ldtk_deser_json::deserialize_project(json.into()) {
            Ok(inner) => Ok(Self { inner }),
            Err(e) => Err(LDTKError::new(format!("Failed to parse the json. {:?}", e))),
        }
    }

    pub fn get_level<T: Into<String>>(&self, level_name: T) -> Option<Level> {
        let level_name: String = level_name.into();

        for level in &self.inner.levels {
            if level.identifier == level_name {
                return Some(Level::new(level.clone()));
            }
        }

        None
    }
}
