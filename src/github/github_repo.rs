#![allow(non_snake_case)]
use std::io::{Error, ErrorKind};
use base64::decode;
use regex::Regex;
use serde::{Deserialize, Serialize};
use crate::github::github_api::request_github_graphql_api;

#[derive(Deserialize, Debug)]
pub struct ResponseRoot {
    data: Data,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    repository: Repository,
}

#[derive(Deserialize, Debug)]
pub struct Repository {
    id: String,
}

#[derive(Serialize, Debug)]
struct Variables {
    owner: String,
    repo: String,
}

pub async fn get_github_repo_id(
    owner: &String,
    repo: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    let query = String::from(
        "query ($owner: String!, $repo: String!) {
           repository(owner: $owner, name: $repo) {
             id
           }
         }",
    );
    let variables = Variables {
        owner: String::from(owner),
        repo: String::from(repo),
    };

    let response = request_github_graphql_api(query, variables).await?;
    if response.status() != 200 {
        return Err(Box::new(Error::new(ErrorKind::Other, "Failed get_github_repo_id")));
    }

    let data = response.json::<ResponseRoot>().await?;
    let raw_id = String::from_utf8(decode(data.data.repository.id).unwrap()).unwrap();

    let regex = Regex::new(r":Repository(?P<repo_id>\d+)$").unwrap();
    let caps = regex.captures(&raw_id).unwrap();
    let repo_id = &caps["repo_id"];
    Ok(String::from(repo_id))
}
