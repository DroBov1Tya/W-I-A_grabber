use reqwest::blocking::Client; // Используем blocking версию клиента
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Serialize;
use std::error::Error;

#[derive(Serialize)]
struct RequestBody {
    computer_name: String,
    big_bit: Vec<String>,
    low_bit: Vec<String>,
}

pub fn req(big_bit: Vec<String>, low_bit: Vec<String>, token: String, computer_name: String) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let body = RequestBody {
        computer_name,
        big_bit,
        low_bit,
    };

    let body_json = serde_json::to_string(&body)?;

    let mut headers = HeaderMap::new();
    headers.insert("X-ZAP-API-Key", HeaderValue::from_str(&token)?);

    let response = client
        .post("http://109.176.30.151:8000/apps/upload")
        .headers(headers)
        .body(body_json)
        .send()?;

    // Вывод статуса ответа
    println!("{}", response.status());

    Ok(())
}