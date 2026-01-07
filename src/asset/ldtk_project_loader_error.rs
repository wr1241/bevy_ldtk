use thiserror::Error;

#[derive(Debug, Error)]
pub enum LDtkProjectLoaderError {
    #[error("encountered io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("deserialize ldtk project json failed: {0}")]
    Deserialize(#[from] serde_json::Error),
}
