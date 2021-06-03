mod github;
mod zenhub;

use crate::github::github_issue::get_github_issue;
use crate::github::github_repo::get_github_repo_id;
use crate::zenhub::workspace::get_zenhub_workspaces;
use crate::zenhub::zenhub_issue::{get_zenhub_issue, move_pipeline};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let owner: String = String::from("Connehito");
    let repo: String = String::from("yorozu-issues");
    let issue_number: i32 = 350;
    // let target_pipeline: String = String::from("");

    let github_issue = get_github_issue(&owner, &repo, &issue_number).await?;
    let repo_id = get_github_repo_id(&owner, &repo).await?;
    let boards = get_zenhub_workspaces(&repo_id).await?;

    // move_pipeline(&workspace_id, &repo_id, &issue_number, &pipeline_id).await?;

    let zenhub_issue = get_zenhub_issue(&repo_id, &issue_number).await?;
    println!("{:#?}", github_issue);
    println!("{:#?}", repo_id);
    println!("{:#?}", boards);
    println!("{:#?}", zenhub_issue);
    Ok(())
}
