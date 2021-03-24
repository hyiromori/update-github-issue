import { IssueInfo } from "../types.ts";

export const checkGitHubIssueUrl = (url: string): IssueInfo => {
  const { origin, pathname } = new URL(url);
  if (origin !== "https://github.com") {
    throw new Error("GitHub の URL ではありません");
  }

  const [organization, repository, resource, number] = pathname.substr(1).split(
    "/",
  );
  if (resource !== "issues") {
    throw new Error("Issue の URL ではありません");
  }

  const issueNumber: number = parseInt(number, 10);
  if (isNaN(issueNumber) && issueNumber > 0) {
    throw new Error("Issue 番号が整数ではありません");
  }

  return { url, organization, repository, issueNumber };
};
