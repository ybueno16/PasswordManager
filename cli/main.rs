use clap::{ App, Arg };
use std::env;
mod commands {
    pub mod add;
}

fn main() {
    //Incialização Password Manager CLI
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
        .get_matches();
    //Adicionar valores em array para depois adicionar no BD
    if let Some(matches) = matches.subcommand_matches("add") {
        let username = matches.value_of("username").unwrap_or("");
        let password = matches.value_of("password").unwrap_or("");
        let purpose = matches.value_of("purpose").unwrap_or_else(|| "");
        commands::add::add_password_record(username, password, purpose);
    }

}