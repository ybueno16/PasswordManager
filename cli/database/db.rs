use rusqlite::{ params, Connection, Result };
pub mod db;
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

pub fn get_password_record(
    conn: &Connection,
    id: i32,
    username: &str,
    password: &str,
    purpose: &str
) -> Result<()> {
    conn.execute(
        "SELECT * FROM PasswordManager WHERE id = ?1 AND username = ?2 AND password = ?3 AND purpose = ?4",
        params![id, username, password, purpose]
    )?;

    Ok(())
}