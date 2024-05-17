use std::path::Path;

use nohash_hasher::IntMap;

mod game_metadata;
use game_metadata::GameMetadata;

use rusqlite::Connection;

pub const METADATA_FILE: &str = "./metadata.db";

pub struct Metadata {
    pub tags: IntMap<u8, String>,
    pub permission_levels: IntMap<u8, String>,
    pub games: IntMap<i64, GameMetadata>,
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
