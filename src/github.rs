#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use std::env;

fn get_authorization_header_for_github() -> String {
    match env::var("GITHUB_ACCESS_TOKEN") {
        Ok(val) => format!("bearer {}", val),
        Err(_err) => ("").to_string(),
    }
}

#[derive(Serialize, Debug)]
pub struct GitHubIssueRequest {
    query: String,
}

#[derive(Deserialize, Debug)]
pub struct ResponseRoot {
    data: Data,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    user: User,
}

#[derive(Deserialize, Debug)]
pub struct User {
    repository: Repository,
}

#[derive(Deserialize, Debug)]
pub struct Repository {
    issue: GitHubIssue,
}

#[derive(Deserialize, Debug)]
pub struct GitHubIssue {
    pub title: String,
    pub url: String,
}

pub async fn get_github_issue(
    issue_number: i32,
) -> Result<GitHubIssue, Box<dyn std::error::Error>> {
    let url = "https://api.github.com/graphql";
    let query = format!(
        "{{
      user(login: \"mryhryki\") {{
        repository(name: \"HOME\"){{
          issue(number: {}) {{
            title
            url
          }}
        }}
      }}
    }}",
        issue_number
    );
    let req = GitHubIssueRequest { query };
    let body: String = serde_json::to_string(&req).unwrap();

    let res = reqwest::Client::new()
        .post(url)
        .header("User-Agent", "github-issue-cli")
        .header("Authorization", get_authorization_header_for_github())
        .body(body)
        .send()
        .await?;
    if res.status() != 200 {
        println!("{:#?}", res.status());
    }

    let data = res.json::<ResponseRoot>().await?;
    Ok(data.data.user.repository.issue)
}
