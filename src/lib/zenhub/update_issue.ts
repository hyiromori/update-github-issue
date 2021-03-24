import { IssueInfo } from "../types.ts";
import { getRepository } from "../github/get_repository.ts";
import { fetchZenHub } from "./common.ts";
import { getBoard } from "./get_board.ts";

interface Args {
  issueNumber: number;
  organization: string;
  repository: string;
  workspaceId: string;
  pipeline: string;
}

// https://github.com/ZenHubIO/API#move-an-issue-between-pipelines
export const movePipeline = async (args: Args): Promise<void> => {
  const { issueNumber, organization, repository, workspaceId, pipeline } = args;
  const { pipelines } = await getBoard(organization, repository, workspaceId);
  const { id: pipelineId } = pipelines.find(({ name }) => name === pipeline) ??
    { id: null };
  if (pipelineId == null) {
    throw new Error(`パイプラインが見つかりません (${pipeline})`);
  }

  const { id: repositoryId } = await getRepository(organization, repository);
  await fetchZenHub(
    "POST",
    `/p2/workspaces/${workspaceId}/repositories/${repositoryId}/issues/${issueNumber}/moves`,
    { pipeline_id: pipelineId, position: "bottom" },
  );
};
