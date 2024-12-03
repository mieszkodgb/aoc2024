use std::fs;

use reqwest;
use reqwest::header::COOKIE;
use std::error;


pub async fn get_vec_input<'a>(url: &str, cookie: &str) -> Result<Vec<String>, reqwest::Error> {
    println!("{}", cookie);
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(COOKIE, format!("session={cookie}"))
        .send()
        .await?;
    println!("Status: {}", response.status());
    let body = response.text().await?;
    let mut input: Vec<String> = body.split("\n").map(|v| v.to_string()).collect::<Vec<String>>();
    input.pop().unwrap();
    return Ok(input);
}

pub async fn get_str_input<'a>(url: &str, cookie: &str) -> Result<String, reqwest::Error> {
    println!("{}", cookie);
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(COOKIE, format!("session={cookie}"))
        .send()
        .await?;
    println!("Status: {}", response.status());
    let body = response.text().await?;
    // let mut input: Vec<String> = body.split("\n").map(|v| v.to_string()).collect::<Vec<String>>();
    // input.pop().unwrap();
    return Ok(body);
}

pub fn get_dummy_input(filename: &str) -> Result<Vec<String>, Box<dyn error::Error>> {
    let message: String = fs::read_to_string(filename)?;
    let input: Vec<String> = message.split("\n").map(|v| v.to_string()).collect::<Vec<String>>();
    return Ok(input);
}