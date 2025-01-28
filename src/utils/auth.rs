use std::fs::File;
use std::io::Write;
use std::path::Path;

const AUTH_FILE: &str = "auth.json";

pub fn save_auth(token: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(Path::new(AUTH_FILE))?;
    file.write_all(token.as_bytes())?;
    Ok(())
}

pub fn load_auth() -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(Path::new(AUTH_FILE))?;
    Ok(content)
}
