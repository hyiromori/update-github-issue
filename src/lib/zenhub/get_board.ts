import { getRepository } from "../github/get_repository.ts";
import { fetchZenHub } from "./common.ts";

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
  return await fetchZenHub(
    "GET",
    `/p2/workspaces/${workspaceId}/repositories/${repositoryId}/board`,
  );
};
