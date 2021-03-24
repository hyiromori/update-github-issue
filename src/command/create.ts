import { createIssue } from "../lib/github/create_issue.ts";
import { getTitle } from "../lib/args.ts";

export const create = async (): Promise<void> => {
  const title = getTitle();
  if (title != null) {
    throw new Error("Issueタイトルを指定してください。");
  }
  // await createIssue(, title)
};
