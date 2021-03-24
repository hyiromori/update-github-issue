import { createIssue } from "../lib/github/create_issue.ts";
import {
  getEpicUrl,
  getLabels,
  getOwner,
  getPipeline,
  getRepository,
  getTitle,
  getWorkspace,
} from "../lib/args.ts";
import { checkGitHubIssueUrl } from "../lib/checker/github_issue.ts";
import { updateIssue } from "../lib/github/update_issue.ts";
import { movePipeline } from "../lib/zenhub/update_issue.ts";
import { addEpic } from "../lib/zenhub/add_epic.ts";

export const create = async (): Promise<void> => {
  const { html_url: url } = await createIssue(
    getOwner(),
    getRepository(),
    getTitle(),
  );
  const issue = checkGitHubIssueUrl(url);

  const epicUrl = getEpicUrl();
  const pipeline = getPipeline();
  if (pipeline != null || epicUrl != null) {
    if (pipeline != null) {
      await movePipeline({
        issueNumber: issue.issueNumber,
        organization: issue.organization,
        repository: issue.repository,
        workspaceId: getWorkspace(),
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
  if (labels != null && labels.length > 0) {
    await updateIssue(issue, { labels });
  }
};
