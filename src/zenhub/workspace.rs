use serde::{Deserialize};
use crate::zenhub::zenhub_api::request_zenhub_api;

#[derive(Deserialize, Debug)]
pub struct Workspace {
    name: String,
    id: String,
}

pub async fn get_workspaces(repo_id: &String) -> Result<Vec<Workspace>, Box<dyn std::error::Error>> {
    // https://github.com/ZenHubIO/API#get-zenhub-workspaces-for-a-repository
    let path = format!("/p2/repositories/{}/workspaces", repo_id);
    let response = request_zenhub_api(&path).await?;
    // let data = response.text().await?;
    // println!("{:#?}", data);
    let data: Vec<Workspace> = response.json::<Vec<Workspace>>().await?;
    Ok(data)
}


