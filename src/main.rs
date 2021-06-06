mod github;
mod util;
mod zenhub;

extern crate dirs;
use crate::github::github_issue::get_github_issue;
use crate::github::github_owners::get_github_owners;
use crate::github::github_repo::get_github_repos;
use crate::github::structs::Owner;
use crate::util::config::{get_config_file_path, read_config, write_config, Config};
use crate::zenhub::board::get_board;
use crate::zenhub::epic::get_epic_issues;
use crate::zenhub::structs::Board;
use crate::zenhub::workspace::get_zenhub_workspaces;
use crate::zenhub::zenhub_issue::{get_zenhub_issue, move_pipeline};
use std::{env, fmt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Vec<String> = env::args().collect();
    let subcommand = String::from(&args[1]);
    let args: Vec<String> = args.split_off(2);

    println!("{:?}", read_config().unwrap().workspace_name);

    let owners = get_github_owners().await?;
    let owner = select_in_menu(&owners);
    match owner {
        None => {
            panic!("Owner not found or unselected.")
        }
        Some(val) => {
            println!("Owner    : {owner_name}", owner_name = val.login);
        }
    }

    let repos = get_github_repos(&owner.unwrap()).await?;
    let repo = select_in_menu(&repos);
    match repo {
        None => {
            panic!("Repo not found or unselected.")
        }
        Some(val) => {
            println!(
                "Repo     : {repo_owner}/{repo_name}",
                repo_owner = val.owner.login,
                repo_name = val.name
            );
        }
    }

    let workspaces = get_zenhub_workspaces(&repo.unwrap().get_repo_id()).await?;
    let workspace = select_in_menu(&workspaces);
    match workspace {
        None => {
            panic!("Workspace not found or unselected.")
        }
        Some(val) => {
            println!(
                "Workspace: {workspace_name} (ID: {workspace_id})",
                workspace_name = val.name,
                workspace_id = val.id
            );
            write_config(&Config {
                workspace_id: String::from(&val.id),
                workspace_name: String::from(&val.name),
            });
            println!("Config saved: {}", get_config_file_path());
        }
    }

    // let workspace_id: String = String::from("606c08cc26504900173dc46e");
    // let pipeline_id: String = String::from("Z2lkOi8vcmFwdG9yL1BpcGVsaW5lLzIzNjU5NTk"); // New Issue
    // let pipeline_id: String = String::from("Z2lkOi8vcmFwdG9yL1BpcGVsaW5lLzIzNjU5Njc"); // Task Current

    // let repo_id = get_github_repo_id(&owner, &repo).await?;
    // let github_issue = get_github_issue(&owner, &repo, &issue_number).await?;
    // let pipelines = get_board(&workspace_id, &repo_id).await?;
    // let epic_issues = get_epic_issues(&repo_id).await?;

    // move_pipeline(&workspace_id, &repo_id, &issue_number, &pipeline_id).await?;

    // let mut menu = youchoose::Menu::new(workspaces.iter());
    // let index: usize = menu.show().first().unwrap().clone();
    // let workspace = &workspaces[index];
    // println!("Index {}: {:#?}", index, workspace);

    // let zenhub_issue = get_zenhub_issue(&repo_id, &issue_number).await?;
    // println!("{:#?}", github_issue);
    // println!("{:#?}", repo_id);
    // println!("{:#?}", boards);
    // println!("{:#?}", zenhub_issue);
    // println!("{:#?}", pipelines);
    // println!("{:#?}", epic_issues);
    Ok(())
}

fn select_in_menu<T: fmt::Display>(collection: &Vec<T>) -> Option<&T> {
    if collection.is_empty() {
        return None;
    }
    let mut menu = youchoose::Menu::new(collection.iter());
    let index: usize = menu.show().first().unwrap().clone();
    Some(&collection[index])
}
