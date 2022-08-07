use http::{Request, Response};
use std::io::stdin;

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

    let mut request = Request::Builder::new()
        .method("POST")
        .body(format!("username={}&password={}", username, password))
        .unwrap();
    request
        .uri("http://localhost:8000/login")
        .header("Content-Type", "application/json");

    let response = send(request.body(()).unwrap());

    fn send(req: Request<()>) -> Response<()> {
        let mut response = req.send().unwrap();
        let body = response.body().unwrap();
        let body = body.into_string().unwrap();
        println!("{}", body);
        response
    }
}
