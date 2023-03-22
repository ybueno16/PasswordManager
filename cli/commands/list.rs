use rusqlite::{ Connection, Result };
use crate::database::db::PasswordRecord;

pub fn list_password_record(conn: &Connection) -> Result<Vec<PasswordRecord>> {
    let mut stmt = conn.prepare("SELECT * FROM PasswordManager")?;
    let records = stmt.query_map([], |row|{
        Ok(PasswordRecord::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
        ))
    })?;

    let mut result = vec![];
    for record in records {
        result.push(record?);
    }
    Ok(result)
}
