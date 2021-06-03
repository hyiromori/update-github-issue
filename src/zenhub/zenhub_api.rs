#![allow(non_snake_case)]
use std::env;
use reqwest::Response;
use serde::Serialize;

// https://github.com/ZenHubIO/API#authentication
fn get_authorization_header() -> String {
    match env::var("ZENHUB_ACCESS_TOKEN") {
        Ok(val) => String::from(val),
        Err(_err) => String::from(""),
    }
}

pub async fn get_zenhub_api(path: &String) -> Result<Response, Box<dyn std::error::Error>> {
    let url = format!("https://api.zenhub.com{}", path);
    let res = reqwest::Client::new()
        .get(url)
        .header("X-Authentication-Token", get_authorization_header())
        .send()
        .await?;
    Ok(res)
}

pub async fn post_zenhub_api<T: Serialize>(
    path: &String,
    req: T,
) -> Result<Response, Box<dyn std::error::Error>> {
    let url = format!("https://api.zenhub.com{}", path);
    let body: String = serde_json::to_string(&req).unwrap();
    let res = reqwest::Client::new()
        .post(url)
        .header("X-Authentication-Token", get_authorization_header())
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;
    if res.status() != 200 {
        println!("{:#?}", res.status());
    }
    Ok(res)
}
