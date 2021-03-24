import { updateIssue } from "../lib/github/update_issue.ts";
import { checkGitHubIssueUrl } from "../lib/checker/github_issue.ts";
import {
  getEpicUrl,
  getLabels,
  getPipeline,
  getUrls,
  getWorkspace,
} from "../lib/args.ts";
import { movePipeline } from "../lib/zenhub/update_issue.ts";
import { addEpic } from "../lib/zenhub/add_epic.ts";

export const update = async (): Promise<void> => {
  const urls = getUrls();
  if (urls.length === 0) {
    throw new Error("URLが指定されていません。");
  }

  await Promise.all(urls.map(async (url: string): Promise<void> => {
    const issue = checkGitHubIssueUrl(url);

    const epicUrl = getEpicUrl();
    const pipeline = getPipeline();
    if (pipeline != null || epicUrl != null) {
      const workspaceId = getWorkspace();
      if (workspaceId == null) {
        throw new Error(
          "ZenHub のデータを更新する場合は --workspace [workspace_id] を指定してください。",
        );
      }

      if (pipeline != null) {
        await movePipeline({
          issueNumber: issue.issueNumber,
          organization: issue.organization,
          repository: issue.repository,
          workspaceId,
          pipeline,
        });
      }

      if (epicUrl != null) {
        const epicIssue = checkGitHubIssueUrl(epicUrl);
        await addEpic({
          organization: issue.organization,
          issueNumber: issue.issueNumber,
          repository: issue.repository,
          epicRepository: epicIssue.repository,
          epicIssueNumber: epicIssue.issueNumber,
        });
      }
    }

    const labels = getLabels();
    if (labels != null) {
      await updateIssue(issue, { labels: labels });
    }
  }));
};
