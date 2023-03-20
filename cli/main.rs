use clap::{ Arg, App };

fn main() {
    let matches = App::new("Password Manager CLI")
        .version("1.0")
        .author("ybueno16 - Yuri Bueno")
        .about("Gerenciador de senhas via SLI")
        .arg(
            Arg::with_name("Username")
                .short("u")
                .long("Username")
                .takes_value(true)
                .help("Nome de usuário")
        )
        .arg(Arg::with_name("password").short("p").long("password").takes_value(true).help("Senha"))
        .arg(
            Arg::with_name("purpose")
                .short("r")
                .long("purpose")
                .takes_value(true)
                .help("Propósito do login")
        )
        .get_matches();
}