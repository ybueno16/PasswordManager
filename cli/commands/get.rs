use rusqlite::{ Connection, Result };

pub fn get(conn: &Connection, id: i32, username: &str, password: &str, purpose: &str) {
    crate::database::db::get_password_record(conn, id, username, password, purpose);
    println!("Select feito com sucesso");
}