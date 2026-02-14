use rusqlite::{Connection, Result};
use std::path::Path;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(SCHEMA)?;
        Ok(Self { conn })
    }

    pub fn add_profile(&self, name: &str) -> Result<()> {
        self.conn
            .execute("INSERT OR IGNORE INTO profiles (name) VALUES (?1)", [name])?;
        Ok(())
    }

    pub fn remove_profile(&self, name: &str) -> Result<bool> {
        let affected = self
            .conn
            .execute("DELETE FROM profiles WHERE name = ?1", [name])?;
        Ok(affected > 0)
    }

    pub fn use_profile(&self, name: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO meta (key, value) VALUES ('active_profile', ?1)",
            [name],
        )?;
        Ok(())
    }

    pub fn get_active_profile(&self) -> Result<Option<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT value FROM meta WHERE key = 'active_profile'")?;
        let mut rows = stmt.query([])?;
        Ok(rows.next()?.map(|row| row.get(0).unwrap()))
    }

    pub fn list_profiles(&self) -> Result<Vec<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT name FROM profiles ORDER BY name")?;
        let names: Vec<String> = stmt
            .query_map([], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();
        Ok(names)
    }

    pub fn add_entry(&self, profile_name: &str, host: &str, address: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO entries (profile_name, host, address) VALUES (?1, ?2, ?3)",
            (profile_name, host, address),
        )?;
        Ok(())
    }

    pub fn remove_entry(&self, profile_name: &str, host: &str) -> Result<bool> {
        let affected = self.conn.execute(
            "DELETE FROM entries WHERE profile_name = ?1 AND host = ?2",
            (profile_name, host),
        )?;
        Ok(affected > 0)
    }

    pub fn update_entry(&self, profile_name: &str, host: &str, address: &str) -> Result<bool> {
        let affected = self.conn.execute(
            "UPDATE entries SET address = ?1 WHERE profile_name = ?2 AND host = ?3",
            (address, profile_name, host),
        )?;
        Ok(affected > 0)
    }

    pub fn get_entries(&self, profile_name: &str) -> Result<Vec<(String, String)>> {
        let mut stmt = self
            .conn
            .prepare("SELECT host, address FROM entries WHERE profile_name = ?1")?;
        let entries = stmt
            .query_map([profile_name], |row| Ok((row.get(0)?, row.get(1)?)))?
            .filter_map(|r| r.ok())
            .collect();
        Ok(entries)
    }
}

const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS profiles (
    name TEXT PRIMARY KEY,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE IF NOT EXISTS entries (
    profile_name TEXT,
    host TEXT,
    address TEXT,
    PRIMARY KEY (profile_name, host),
    FOREIGN KEY (profile_name) REFERENCES profiles(name) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS meta (
    key TEXT PRIMARY KEY,
    value TEXT
);
INSERT OR IGNORE INTO meta (key, value) VALUES ('active_profile', 'default');
"#;
