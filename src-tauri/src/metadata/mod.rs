use std::path::Path;

use gamercade_interface::game::{GameInfoBasic, GameInfoDetailed};
use nohash_hasher::IntMap;

mod game_metadata;
use game_metadata::GameMetadata;

use rusqlite::Connection;

pub const METADATA_FILE: &str = "./metadata.db";

pub struct Metadata {
    tags: IntMap<u8, String>,
    permission_levels: IntMap<u8, String>,
    games: IntMap<i64, GameMetadata>,
    connection: Connection,
}

impl Metadata {
    pub fn new() -> Self {
        let connection = if Path::new(METADATA_FILE).exists() {
            Connection::open(METADATA_FILE).unwrap()
        } else {
            let connection = Connection::open(METADATA_FILE).unwrap();

            init_db(&connection);

            connection
        };

        Self {
            permission_levels: IntMap::default(),
            tags: IntMap::default(),
            games: IntMap::default(),
            connection,
        }
    }

    // TODO: Write a func to handle the global refresh updates for:
    // - permission_levels
    // - tags

    pub fn update_game_basic(&mut self, game_info: &GameInfoBasic) {
        // TODO: This
    }

    pub fn update_game_detailed(&mut self, game_info: &GameInfoDetailed) {
        if let Some(basic) = &game_info.basic_info {
            self.update_game_basic(basic)
        };

        // TODO: This
    }

    pub fn delete_game(&mut self, game_id: i64) {
        self.games.remove(&game_id);
    }
}

fn init_db(conn: &Connection) {
    // Init games
    conn.execute(
        "CREATE TABLE IF NOT EXISTS games (
        id INTEGER PRIMARY KEY,
        value TEXT NOT NULL,
        last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
    );",
        (),
    )
    .unwrap();

    // Init Last Updated
    conn.execute(
        "CREATE TABLE IF NOT EXISTS timestamps (
            table_name TEXT NOT NULL,
            last_updated TIMESTAMP NOT NULL
        );",
        (),
    )
    .unwrap();

    // Init Permission Levels
    conn.execute(
        "CREATE TABLE IF NOT EXISTS permission_levels (
        id INTEGER PRIMARY KEY,
        value TEXT NOT NULL
    );",
        (),
    )
    .unwrap();

    // Init Tags
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
        id INTEGER PRIMARY KEY,
        value TEXT NOT NULL
    );",
        (),
    )
    .unwrap();
}
