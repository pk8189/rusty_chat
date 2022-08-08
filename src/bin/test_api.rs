use std::{collections::HashMap, io::stdin};

use serde::Deserialize;

#[derive(Deserialize)]
struct LoginResponse {
    pub token: String,
}

pub type Result<T, CustomErr> = std::result::Result<T, CustomErr>;

fn get_token(username: &str, password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("username", username);
    map.insert("password", password);
    let client = reqwest::blocking::Client::new();
    let resp = client
        .post("http://localhost:8000/login")
        .json(&map)
        .send()?;

    if resp.status().is_success() {
        println!("successfully retieved JWT");
        let res: LoginResponse = resp.json()?;
        return Ok(res.token);
    } else {
        println!("Unable to log in with the provided credentials");
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unable to log in with the provided credentials",
        )
        .into());
    }
}

#[allow(dead_code)]
fn main() {
    println!("Username:");
    let mut username = String::new();
    stdin().read_line(&mut username).unwrap();
    let username = &username[..(username.len() - 1)]; // Drop the newline character
    println!("Password");
    let mut password = String::new();
    stdin().read_line(&mut password).unwrap();
    let password = &password[..(password.len() - 1)]; // Drop the newline character
    match get_token(username, password) {
        Ok(token) => println!("{}", token),
        Err(e) => println!("{}", e),
    }
    return;
}
