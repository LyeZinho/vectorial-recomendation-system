//! SQLite storage layer for harvested entities
use rusqlite::{Connection, Result as SqlResult};
use uuid::Uuid;

pub struct SqliteStorage {
    conn: Connection,
}

impl SqliteStorage {
    pub fn new(db_path: &str) -> SqlResult<Self> {
        let conn = Connection::open(db_path)?;
        let storage = SqliteStorage { conn };
        storage.init_schema()?;
        Ok(storage)
    }

    fn init_schema(&self) -> SqlResult<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS entities (
                internal_id TEXT PRIMARY KEY,
                entity_type TEXT NOT NULL,
                primary_name TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
            
            CREATE TABLE IF NOT EXISTS external_ids (
                id INTEGER PRIMARY KEY,
                internal_id TEXT NOT NULL,
                source TEXT NOT NULL,
                external_id TEXT NOT NULL,
                UNIQUE(source, external_id),
                FOREIGN KEY(internal_id) REFERENCES entities(internal_id)
            );
            
            CREATE TABLE IF NOT EXISTS anime_metadata (
                internal_id TEXT PRIMARY KEY,
                title TEXT,
                score REAL,
                episodes INTEGER,
                genres TEXT,
                studios TEXT,
                FOREIGN KEY(internal_id) REFERENCES entities(internal_id)
            );
            
            CREATE TABLE IF NOT EXISTS person_metadata (
                internal_id TEXT PRIMARY KEY,
                given_name TEXT,
                family_name TEXT,
                birthday TEXT,
                FOREIGN KEY(internal_id) REFERENCES entities(internal_id)
            );
            
            CREATE TABLE IF NOT EXISTS manga_metadata (
                internal_id TEXT PRIMARY KEY,
                title TEXT,
                volumes INTEGER,
                chapters INTEGER,
                FOREIGN KEY(internal_id) REFERENCES entities(internal_id)
            );
            
            CREATE TABLE IF NOT EXISTS user_metadata (
                internal_id TEXT PRIMARY KEY,
                username TEXT,
                days_watched REAL,
                FOREIGN KEY(internal_id) REFERENCES entities(internal_id)
            );
            
            CREATE TABLE IF NOT EXISTS harvest_log (
                id INTEGER PRIMARY KEY,
                file_path TEXT NOT NULL,
                entity_type TEXT NOT NULL,
                confidence REAL NOT NULL,
                rows_parsed INTEGER NOT NULL,
                harvested_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
            
            CREATE TABLE IF NOT EXISTS triplets (
                id INTEGER PRIMARY KEY,
                subject_id TEXT NOT NULL,
                predicate TEXT NOT NULL,
                object_str TEXT NOT NULL,
                weight REAL,
                FOREIGN KEY(subject_id) REFERENCES entities(internal_id)
            );",
        )?;
        Ok(())
    }

    pub fn insert_entity(
        &self,
        internal_id: &Uuid,
        entity_type: &str,
        primary_name: &str,
    ) -> SqlResult<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO entities (internal_id, entity_type, primary_name) VALUES (?, ?, ?)",
            [internal_id.to_string(), entity_type.to_string(), primary_name.to_string()],
        )?;
        Ok(())
    }

    pub fn insert_external_id(
        &self,
        internal_id: &Uuid,
        source: &str,
        external_id: &str,
    ) -> SqlResult<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO external_ids (internal_id, source, external_id) VALUES (?, ?, ?)",
            [internal_id.to_string(), source.to_string(), external_id.to_string()],
        )?;
        Ok(())
    }

    pub fn insert_harvest_log(
        &self,
        file_path: &str,
        entity_type: &str,
        confidence: f64,
        rows_parsed: i32,
    ) -> SqlResult<()> {
        self.conn.execute(
            "INSERT INTO harvest_log (file_path, entity_type, confidence, rows_parsed) VALUES (?, ?, ?, ?)",
            [file_path, entity_type, &confidence.to_string(), &rows_parsed.to_string()],
        )?;
        Ok(())
    }

    pub fn table_exists(&self, table_name: &str) -> SqlResult<bool> {
        let mut stmt = self
            .conn
            .prepare("SELECT 1 FROM sqlite_master WHERE type='table' AND name=?1")?;
        Ok(stmt.exists([table_name])?)
    }
}
