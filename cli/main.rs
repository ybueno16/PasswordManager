mod commands {
    pub mod add;
    pub mod remove;
    pub mod get;
}

mod database {
    pub mod db;
}

use clap::{ App, Arg };
use rusqlite::{ Connection, Result };
use database::db::create_table;

fn main() -> Result<(), rusqlite::Error> {
    let conn = Connection::open("cli/database/Manager.sqlite3")?;
    create_table(&conn)?;

    //Inicialização Password Manager CLI
    let matches = App::new("Password Manager CLI")
        .author("ybueno16 - Yuri Bueno")
        .about("Gerenciador de senhas via CLI")
        .subcommand(
            App::new("add")
                .about("Adicionar novo registro de usuário e senha")
                .arg(
                    Arg::with_name("username")
                        .short('u')
                        .long("username")
                        .takes_value(true)
                        .help("Nome de usuário")
                )

                .arg(
                    Arg::with_name("password")
                        .short('p')
                        .long("password")
                        .takes_value(true)
                        .help("Senha")
                )
                .arg(
                    Arg::with_name("purpose")
                        .short('r')
                        .long("purpose")
                        .takes_value(true)
                        .help("Propósito do login")
                )
        )
        .subcommand(
            App::new("remove")
                .about("Remover registro de usuário e senha")
                .arg(
                    Arg::with_name("id")
                        .short('i')
                        .long("id")
                        .required(true)
                        .takes_value(true)
                        .help("ID do registro a ser removido")
                )
        )

        //Colocar comando get
        .get_matches();

    //Adicionar valores em array para depois adicionar no BD
    if let Some(matches) = matches.subcommand_matches("add") {
        let username = matches.value_of("username").unwrap_or("");
        let password = matches.value_of("password").unwrap_or("");
        let purpose = matches.value_of("purpose").unwrap_or_else(|| "");
        commands::add::add_password_record(username, password, purpose);
        database::db::add_password_record(&conn, username, password, purpose)?;
        println!("Registro adicionado com sucesso.");
    }

    if let Some(matches) = matches.subcommand_matches("remove") {
        let id = matches.value_of("id").unwrap().parse::<i32>().unwrap();
        commands::remove::remove(&conn, id)?;
        println!("Registro removido com sucesso.");
    }

    if let Some(matches) = matches.subcommand_matches("get") {
        let id = matches.value_of("id").unwrap().parse::<i32>().unwrap();
        let username = matches.value_of("username").unwrap_or("");
        let password = matches.value_of("password").unwrap_or("");
        let purpose = matches.value_of("purpose").unwrap_or_else(|| "");
        commands::get::get(&conn, id, username, password, purpose);
    }

    Ok(())
}