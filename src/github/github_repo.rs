#![allow(non_snake_case)]
use crate::github::github_api::{get_github_api_v3, request_github_graphql_api};
use crate::github::structs::{Owner, OwnerForRepo, OwnerType, Repository};
use base64::decode;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};

#[derive(Deserialize, Debug)]
pub struct UserResponse {
    data: UserData,
}

#[derive(Deserialize, Debug)]
pub struct UserData {
    user: UserOrOrganizationData,
}

#[derive(Deserialize, Debug)]
pub struct OrganizationResponse {
    data: OrganizationData,
}

#[derive(Deserialize, Debug)]
pub struct OrganizationData {
    organization: UserOrOrganizationData,
}

#[derive(Deserialize, Debug)]
pub struct UserOrOrganizationData {
    repositories: Repositories,
}

#[derive(Deserialize, Debug)]
pub struct Repositories {
    nodes: Vec<RepositoryForGraphQL>,
}

#[derive(Deserialize, Debug)]
pub struct RepositoryForGraphQL {
    pub id: String,
    pub name: String,
    pub owner: OwnerForRepo,
}

impl RepositoryForGraphQL {
    pub fn get_repo_id(&self) -> i32 {
        let raw_id = String::from_utf8(decode(&self.id).unwrap()).unwrap();
        let regex = Regex::new(r":Repository(?P<repo_id>\d+)$").unwrap();
        let caps = regex.captures(&raw_id).unwrap();
        let repo_id = &caps["repo_id"];
        String::from(repo_id).parse::<i32>().unwrap()
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct Variables {
    owner: String,
}

pub async fn get_github_repos(
    owner: &Owner,
) -> Result<Vec<Repository>, Box<dyn std::error::Error>> {
    let owner_type: String = match owner.owner_type {
        OwnerType::User => String::from("user"),
        OwnerType::Organization => String::from("organization"),
    };
    let query = format!(
        "query ($owner: String!) {{
          {owner_type} (login: $owner) {{
            repositories(first: 100, orderBy: {{ field: UPDATED_AT, direction: DESC }}){{
              nodes {{
                id
                name
                owner {{
                  login
                }}
              }}
            }}
          }}
        }}",
        owner_type = owner_type
    );
    let variables = Variables {
        owner: String::from(&owner.login),
    };
    let response = request_github_graphql_api(query, variables).await?;
    if response.status() != 200 {
        return Err(Box::new(Error::new(
            ErrorKind::Other,
            "Failed get_github_repo_id",
        )));
    }
    let data: Vec<RepositoryForGraphQL> = match owner.owner_type {
        OwnerType::User => {
            let data = response.json::<UserResponse>().await?;
            data.data.user.repositories.nodes
        }
        OwnerType::Organization => {
            let data = response.json::<OrganizationResponse>().await?;
            data.data.organization.repositories.nodes
        }
    };

    let data: Vec<Repository> = data
        .iter()
        .map(|repo| Repository {
            id: repo.get_repo_id(),
            name: repo.name.clone(),
            owner: repo.owner.clone(),
        })
        .collect();

    Ok(data)
}

pub async fn get_github_repo_by_id(
    repo_id: &i32,
) -> Result<Repository, Box<dyn std::error::Error>> {
    let path = format!("/repositories/{repo_id}", repo_id = &repo_id);
    let res = get_github_api_v3(&path).await?;
    let data = res.json::<Repository>().await?;
    Ok(data)
}
