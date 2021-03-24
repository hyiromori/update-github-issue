import { getGitHubAccessToken } from "../env.ts";
import { verbose } from "../args.ts";

const GitHubURL = "https://api.github.com";

export interface CreateIssueData {
  title: string;
  labels?: string[];
}

// https://docs.github.com/en/rest/reference/issues#create-an-issue
export const createIssue = async (
  owner: string,
  repository: string,
  data: CreateIssueData,
): Promise<void> => {
  const path = `/repos/${owner}/${repository}/issues`;
  const response = await fetch(`${GitHubURL}${path}`, {
    method: "POST",
    headers: {
      Authorization: `token ${getGitHubAccessToken()}`,
      Accept: "application/vnd.github.v3+json",
    },
    body: JSON.stringify(data),
  });

  if (!response.ok) {
    throw new Error("GitHub の Issue データの更新に失敗しました。");
  }
  if (verbose()) {
    const responseData = await response.json();
    console.log("Response data:", JSON.stringify(responseData, null, 2));
  }
};
