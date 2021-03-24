export const getGitHubAccessToken = (): string => {
  const token = Deno.env.get("GITHUB_ACCESS_TOKEN");
  if (token != null) {
    return token;
  }
  throw new Error("GITHUB_ACCESS_TOKEN を指定してください");
};

export const getZenHubAccessToken = (): string => {
  const token = Deno.env.get("ZENHUB_ACCESS_TOKEN");
  if (token != null) {
    return token;
  }
  throw new Error("ZENHUB_ACCESS_TOKEN を指定してください");
};
