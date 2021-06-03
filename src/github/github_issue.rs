#![allow(non_snake_case)]
use std::io::{Error, ErrorKind};
use serde::{Deserialize, Serialize};
use crate::github::github_api::request_github_graphql_api;

#[derive(Deserialize, Debug)]
struct ResponseRoot {
    data: Data,
}

#[derive(Deserialize, Debug)]
struct Data {
    repository: Repository,
}

#[derive(Deserialize, Debug)]
struct Repository {
    issue: GitHubIssue,
}

#[derive(Deserialize, Debug)]
pub struct GitHubIssue {
    pub body: String,
    pub createdAt: String,
    pub number: i32,
    pub title: String,
    pub updatedAt: String,
    pub url: String,
}

#[derive(Serialize, Debug)]
struct Variables {
    owner: String,
    repo: String,
    issue_number: i32,
}

pub async fn get_github_issue(
    owner: &String,
    repo: &String,
    issue_number: &i32,
) -> Result<GitHubIssue, Box<dyn std::error::Error>> {
    let query = String::from(
        "query ($owner: String!, $repo: String!, $issue_number: Int!) {
           repository(owner: $owner, name: $repo) {
             issue(number: $issue_number) {
               body
               createdAt
               number
               title
               updatedAt
               url
             }
           }
         }",
    );
    let variables = Variables {
        owner: String::from(owner),
        repo: String::from(repo),
        issue_number: issue_number.clone(),
    };

    let response = request_github_graphql_api(query, variables).await?;
    if response.status() == 200 {
        let data = response.json::<ResponseRoot>().await?;
        Ok(data.data.repository.issue)
    } else {
        Err(Box::new(Error::new(ErrorKind::Other, "Failed get_github_issue")))
    }
}
