use rusqlite::{Connection, Result};


fn main() -> Result<()> {

    let conn = Connection::open("Manager.rs")?;
    let mut stmt = conn.prepare("SELECT * FROM PasswordManager;")?;


}
