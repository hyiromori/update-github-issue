#![allow(non_snake_case)]
use crate::github::github_api::request_github_graphql_api;
use serde::{Deserialize, Serialize};

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
    issue: i32,
}

pub async fn get_github_issue(
    owner: &String,
    repo: &String,
    issue: &i32,
) -> Result<GitHubIssue, Box<dyn std::error::Error>> {
    let query = String::from(
        "query ($owner: String!, $repo: String!, $issue: Int!) {
           repository(owner: $owner, name: $repo) {
             issue(number: $issue) {
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
        issue: issue.clone(),
    };

    let response = request_github_graphql_api(query, variables).await?;
    let data = response.json::<ResponseRoot>().await?;
    Ok(data.data.repository.issue)
}
