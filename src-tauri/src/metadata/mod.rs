use std::path::Path;

use gamercade_interface::{
    author::PermissionLevel,
    game::{GameInfoBasic, GameInfoDetailed},
    tag::Tag,
};
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

        // TOOD: Populate the fields from the db

        Self {
            permission_levels: IntMap::default(),
            tags: IntMap::default(),
            games: IntMap::default(),
            connection,
        }
    }

    pub fn update_global_tags(&mut self, tags: &[Tag]) {
        self.connection.execute("DROP TABLE tags", ()).unwrap();
        create_tags_table(&self.connection);

        self.tags.clear();
        tags.iter().for_each(|tag| {
            self.connection
                .execute(
                    "INSERT INTO tags (id, value) VALUES (?, ?) ",
                    (tag.pid, &tag.name),
                )
                .unwrap();
            self.tags.insert(tag.pid as u8, tag.name.clone());
        });
    }

    pub fn update_global_permission_levels(&mut self, levels: &[PermissionLevel]) {
        self.connection
            .execute("DROP TABLE permission_levels", ())
            .unwrap();
        create_permission_levels_table(&self.connection);

        self.permission_levels.clear();
        levels.iter().for_each(|level| {
            self.connection
                .execute(
                    "INSERT INTO permission_levels (id, value) VALUES (?, ?) ",
                    (level.id, &level.level_name),
                )
                .unwrap();
            self.permission_levels
                .insert(level.id as u8, level.level_name.clone());
        });
    }

    pub fn update_game_basic(&mut self, game_info: &GameInfoBasic) {
        // TODO: This, use json-patch
    }

    pub fn update_game_detailed(&mut self, game_info: &GameInfoDetailed) {
        if let Some(basic) = &game_info.basic_info {
            self.update_game_basic(basic)
        };

        // TODO: This use json-patch
    }

    pub fn delete_game(&mut self, game_id: i64) {
        self.games.remove(&game_id);
    }
}

fn create_tags_table(conn: &Connection) {
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

fn create_permission_levels_table(conn: &Connection) {
    // Init Permission Levels
    conn.execute(
        "CREATE TABLE IF NOT EXISTS permission_levels (
        id INTEGER PRIMARY KEY,
        value TEXT NOT NULL
    );",
        (),
    )
    .unwrap();
}

fn init_db(conn: &Connection) {
    create_tags_table(conn);
    create_permission_levels_table(conn);

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
}
