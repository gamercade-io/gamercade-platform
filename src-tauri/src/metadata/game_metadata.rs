use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GameMetadata {
    title: String,
    short_description: String,
    long_description: Option<String>,
    file_checksum: Option<i64>,
    rom_size: Option<u32>,
    tags: Vec<i32>,
}

impl GameMetadata {}
