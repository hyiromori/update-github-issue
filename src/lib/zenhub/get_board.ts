import { getRepository } from "../github/get_repository.ts";
import { fetchZenHub } from "./common.ts";
import {debugLog} from "../logger.ts";

interface Board {
  pipelines: {
    id: string;
    name: string;
  }[];
}

// https://github.com/ZenHubIO/API#get-a-zenhub-board-for-a-repository
export const getBoard = async (
  organization: string,
  repository: string,
  workspaceId: string,
): Promise<Board> => {
  const { id: repositoryId } = await getRepository(organization, repository);
  const board: Board = await fetchZenHub(
    "GET",
    `/p2/workspaces/${workspaceId}/repositories/${repositoryId}/board`,
  );

  debugLog(`Board: ${JSON.stringify(board)}`)
  return board
};
