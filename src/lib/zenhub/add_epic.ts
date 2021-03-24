import { IssueInfo } from "../types.ts";
import { getRepository } from "../github/get_repository.ts";
import { fetchZenHub } from "./common.ts";
import { getBoard } from "./get_board.ts";

interface Args {
  organization: string;
  repository: string;
  issueNumber: number;
  epicRepository: string;
  epicIssueNumber: number;
}

// https://github.com/ZenHubIO/API#add-or-remove-issues-to-epic
export const addEpic = async (args: Args): Promise<void> => {
  const {
    organization,
    repository,
    issueNumber,
    epicRepository,
    epicIssueNumber,
  } = args;
  const { id: repositoryId } = await getRepository(organization, repository);
  const { id: epicRepositoryId } = await getRepository(
    organization,
    epicRepository,
  );
  await fetchZenHub(
    "POST",
    `/p1/repositories/${epicRepositoryId}/epics/${epicIssueNumber}/update_issues`,
    {
      add_issues: [{
        repo_id: repositoryId,
        issue_number: issueNumber,
      }],
      remove_issues: [],
    },
  );
};
