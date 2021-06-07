use crate::github::github_repo::get_github_repo_by_id;
use crate::github::structs::Repository;
use crate::util::config::Config;
use crate::util::select::select_in_menu;
use crate::zenhub::board::get_pipelines;
use std::future::Future;

#[derive(Debug, Clone)]
enum BoardAction {
    Pipeline,
}

impl ToString for BoardAction {
    fn to_string(&self) -> String {
        match self {
            BoardAction::Pipeline => String::from("Pipeline"),
        }
    }
}

pub async fn board(config: &Config, _args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let actions = vec![BoardAction::Pipeline];
    let action = select_in_menu(&String::from("Choose action:"), &actions).unwrap();

    match action {
        BoardAction::Pipeline => {
            let repo_ids = &config.workspace.repositories;

            // TODO: Parallel request
            let mut repositories: Vec<Repository> = vec![];
            for repo_id in repo_ids {
                repositories.push(get_github_repo_by_id(&repo_id).await?);
            }
            println!("{:#?}", repositories);

            // let pipelines = get_pipelines(&config.workspace.id, &repo.id).await?;
            // let pipeline = select_in_menu(&String::from("Select pipeline"), &pipelines);
            // println!("{:#?}", pipeline);
        }
    };

    Ok(())
}
