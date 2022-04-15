#[derive(Debug, Clone)]
pub struct LDTKError {
    pub message: String,
}

impl LDTKError {
    pub fn new<T: Into<String>>(msg: T) -> Self {
        Self {
            message: msg.into(),
        }
    }
}
