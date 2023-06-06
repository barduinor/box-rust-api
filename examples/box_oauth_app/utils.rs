use openapi::models::AccessToken;

use std::{
    error::Error,
    fs::{self, File},
    io::BufReader,
};

pub fn save_access_token(access_token: AccessToken) {
    let data = serde_json::to_string_pretty(&access_token).unwrap();
    fs::write(".access_token.json", data).expect("Unable to save access token");
}

#[allow(dead_code)]
pub fn read_access_token() -> Result<AccessToken, Box<dyn Error>> {
    let file = File::open(".access_token.json")?;
    let reader = BufReader::new(file);
    let access_token = serde_json::from_reader(reader)?;
    Ok(access_token)
}
