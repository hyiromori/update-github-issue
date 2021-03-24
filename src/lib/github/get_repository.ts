import { fetchGitHubGraphQL } from "./graphql.ts";
import { decodeBase64 } from "../base64.ts";

interface Repository {
  id: number;
}

const cache: { [key: string]: Repository } = {};

export const getRepository = async (
  organization: string,
  repository: string,
): Promise<Repository> => {
  const cacheKey = `${organization}/${repository}`;
  if (cache[cacheKey] != null) {
    return cache[cacheKey];
  }
  const json = await fetchGitHubGraphQL(`
    {
      organization(login: "${organization}") {
        repo(name: "${repository}") {
          id
        }
      }
    }`);
  const id: number = parseInt(
    (decodeBase64(json.data.organization.repo.id).match(/[0-9]+$/) ??
      [""])[0],
    10,
  );

  const data: Repository = { id };
  cache[cacheKey] = data;
  return data;
};
