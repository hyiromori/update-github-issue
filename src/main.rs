mod github;
mod zenhub;

use crate::github::github_issue::get_github_issue;
use crate::github::github_repo::get_github_repo_id;
use crate::zenhub::workspace::get_workspaces;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let owner: String = String::from("Connehito");
    let repo: String = String::from("mamari-q-admin");
    let issue: i32 = 1608;

    let issue = get_github_issue(&owner, &repo, &issue).await?;
    let repo_id = get_github_repo_id(&owner, &repo).await?;
    let boards = get_workspaces(&repo_id).await?;
    println!("{:#?}", issue);
    println!("{:#?}", repo_id);
    println!("{:#?}", boards);
    Ok(())
}
