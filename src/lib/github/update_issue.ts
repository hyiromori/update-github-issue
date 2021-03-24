import { IssueInfo } from "../types.ts";
import { getGitHubAccessToken } from "../env.ts";
import { verbose } from "../args.ts";
import {debugLog, infoLog} from "../logger.ts";

const GitHubURL = "https://api.github.com";

export interface UpdateIssueData {
  labels?: string[];
}

// https://docs.github.com/en/rest/reference/issues#update-an-issue
export const updateIssue = async (
  issue: IssueInfo,
  data: UpdateIssueData,
): Promise<void> => {
  const { organization, repository, issueNumber } = issue;
  const path = `/repos/${organization}/${repository}/issues/${issueNumber}`;
  const response = await fetch(`${GitHubURL}${path}`, {
    method: "PATCH",
    headers: {
      Authorization: `token ${getGitHubAccessToken()}`,
      Accept: "application/vnd.github.v3+json",
    },
    body: JSON.stringify(data),
  });

  if (!response.ok) {
    throw new Error("GitHub の Issue データの更新に失敗しました。");
  }

  infoLog(`Updated Issue: ${issue.url}`);
  debugLog(`Update data: ${JSON.stringify(data)}`)
  debugLog(`Response data: ${JSON.stringify(await response.json(), null, 2)}`);
};
