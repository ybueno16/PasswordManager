use rusqlite::{ Connection, Result };

pub fn remove(conn: &Connection, id: i32) -> Result<()> {
    crate::database::db::delete_password_record(conn, id)?;
    println!("Registro de id {} removido com sucesso.", id);
    Ok(())
}