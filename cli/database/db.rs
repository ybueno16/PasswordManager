use rusqlite::{ params, Connection, Result };
use anyhow::{Error as AnyhowError, Result};
pub struct PasswordRecord {
    id: i32,
    username: String,
    password: String,
    purpose: String,
}

impl PasswordRecord {
    pub fn new(id: i32, username: String, password: String, purpose: String) -> Self {
        PasswordRecord {
            id,
            username,
            password,
            purpose,
        }
    }
}

pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS PasswordManager (
            id              INTEGER PRIMARY KEY,
            username        TEXT NOT NULL,
            password        TEXT NOT NULL,
            purpose         TEXT NOT NULL
        )",
        []
    )?;
    Ok(())
}

pub fn add_password_record(
    conn: &Connection,
    username: &str,
    password: &str,
    purpose: &str
) -> Result<()> {
    conn.execute(
        "INSERT INTO PasswordManager (username, password, purpose) VALUES (?1, ?2, ?3)",
        params![username, password, purpose]
    )?;
    Ok(())
}

pub fn delete_password_record(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM PasswordManager WHERE id = ?", [id])?;
    Ok(())
}

pub fn list_password_record(conn: &Connection, id: i32) -> Result<Vec<PasswordRecord>> {
    let mut stmt = conn.prepare("SELECT id, username, password, purpose FROM password_records WHERE id = ?1")?;
    let password_iter = stmt.query_map([id], |row| {
        Ok(PasswordRecord {
            id: row.get(0)?,
            username: row.get(1)?,
            password: row.get(2)?,
            purpose: row.get(3)?,
        })
    })?;

    let mut password_records = Vec::new();
    for password in password_iter {
        password_records.push(password?);
    }

    if password_records.is_empty() {
        return Err(anyhow::Error::msg(format!("No password record found with ID {}", id)));
    }

    Ok(password_records)
}