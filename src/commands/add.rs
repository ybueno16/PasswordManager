#[derive(Debug)]
pub struct PasswordRecorder {
    pub username: String,
    pub password: String,
    pub purpose: String,
}

pub fn add_password_record(username: &str, password: &str, purpose: &str) {
    let recorder = PasswordRecorder {
        username: username.to_string(),
        password: password.to_string(),
        purpose: purpose.to_string(),
    };
    println!("{:?}", recorder);
}