use std::io::{Error, ErrorKind};
use serde::Deserialize;
use crate::zenhub::zenhub_api::get_zenhub_api;

#[derive(Deserialize, Debug)]
pub struct Workspace {
    name: String,
    id: String,
}

pub async fn get_zenhub_workspaces(
    repo_id: &String,
) -> Result<Vec<Workspace>, Box<dyn std::error::Error>> {
    // https://github.com/ZenHubIO/API#get-zenhub-workspaces-for-a-repository
    let path = format!("/p2/repositories/{}/workspaces", repo_id);
    let response = get_zenhub_api(&path).await?;
    if response.status() == 200 {
        let data: Vec<Workspace> = response.json::<Vec<Workspace>>().await?;
        Ok(data)
    } else {
        Err(Box::new(Error::new(ErrorKind::Other, "Failed move_pipeline")))
    }
}
