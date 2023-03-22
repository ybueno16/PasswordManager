use rusqlite::{ params, Connection, Result };

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

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn purpose(&self) -> &str {
        &self.purpose
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

pub fn list_password_record(conn: &Connection) -> Result<Vec<PasswordRecord>> {
    let mut stmt = conn.prepare("SELECT * FROM PasswordManager")?;
    let records = stmt.query_map([], |row| {
        Ok(PasswordRecord {
            id: row.get(0)?,
            username: row.get(1)?,
            password: row.get(2)?,
            purpose: row.get(3)?,
        })
    })?;

    let mut result = vec![];
    for record in records {
        result.push(record?);
    }
    Ok(result)
}