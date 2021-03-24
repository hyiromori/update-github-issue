import { getGitHubAccessToken } from "../env.ts";
import { verbose } from "../args.ts";
import { debugLog, infoLog } from "../logger.ts";

const GitHubURL = "https://api.github.com";

export interface IssueData {
  html_url: string;
}

// https://docs.github.com/en/rest/reference/issues#create-an-issue
export const createIssue = async (
  owner: string,
  repository: string,
  title: string,
): Promise<IssueData> => {
  const path = `/repos/${owner}/${repository}/issues`;
  const response = await fetch(`${GitHubURL}${path}`, {
    method: "POST",
    headers: {
      Authorization: `token ${getGitHubAccessToken()}`,
      Accept: "application/vnd.github.v3+json",
    },
    body: JSON.stringify({ title }),
  });

  if (!response.ok) {
    throw new Error("GitHub Issue の作成に失敗しました。");
  }

  const issue: IssueData = await response.json();
  infoLog(`Created Issue: ${title} (${issue.html_url})`);
  debugLog(`Response data: ${JSON.stringify(issue, null, 2)}`);

  return issue;
};
