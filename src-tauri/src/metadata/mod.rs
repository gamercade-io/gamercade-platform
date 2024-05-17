use nohash_hasher::IntMap;

mod game_metadata;
use game_metadata::GameMetadata;

pub struct Metadata {
    pub tags: IntMap<u8, String>,
    pub permission_levels: IntMap<u8, String>,
    pub games: IntMap<i64, GameMetadata>,
}

impl Metadata {
    pub fn new() -> Self {
        // TODO: This
        Self {
            permission_levels: IntMap::default(),
            tags: IntMap::default(),
            games: IntMap::default(),
        }
    }
}
