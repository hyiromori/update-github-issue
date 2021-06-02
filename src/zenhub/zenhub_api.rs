#![allow(non_snake_case)]
use reqwest::Response;
use std::env;

// https://github.com/ZenHubIO/API#authentication
fn get_authorization_header() -> String {
    match env::var("ZENHUB_ACCESS_TOKEN") {
        Ok(val) => String::from(val),
        Err(_err) => String::from(""),
    }
}

pub async fn request_zenhub_api(path: &String) -> Result<Response, Box<dyn std::error::Error>> {
    let url = format!("https://api.zenhub.com{}", path);
    let res = reqwest::Client::new()
        .get(url)
        // .header("User-Agent", "mryhryki/github-issue-cli")
        .header("X-Authentication-Token", get_authorization_header())
        .send()
        .await?;
    if res.status() != 200 {
        println!("{:#?}", res.status());
    }

    Ok(res)
}
