use rusqlite::{ Connection, Result };

pub fn create_table(conn: &Connection) -> Result<()> {
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS PasswordManager (
                id                  INTEGER PRIMARY KEY,
                name                TEXT NOT NULL,
                email               TEXT NOT NULL,
                purpose             TEXT      
        )",
        []
    )?;

    println!("Banco de dados conectado");
    Ok(())
}