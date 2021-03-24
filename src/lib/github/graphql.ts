import { getGitHubAccessToken } from "../env.ts";

const GitHubURL = "https://api.github.com/graphql";

// https://docs.github.com/en/free-pro-team@latest/graphql/overview/explorer
export const fetchGitHubGraphQL = async (query: string): Promise<any> => {
  const response = await fetch(GitHubURL, {
    method: "POST",
    headers: {
      Authorization: `token ${getGitHubAccessToken()}`,
    },
    body: JSON.stringify({ query }),
  });
  if (response.status !== 200) {
    throw new Error(`Invalid status code: ${response.status}`);
  }
  return response.json();
};
