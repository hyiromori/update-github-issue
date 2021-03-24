import { getZenHubAccessToken } from "../env.ts";

const ZenHubURL = "https://api.zenhub.com";

// https://github.com/ZenHubIO/API
export const fetchZenHub = async (
  method: string,
  path: string,
  body: object = {},
): Promise<any> => {
  const isRead = ["GET", "HEAD"].includes(method.toUpperCase());
  const response = await fetch(`${ZenHubURL}${path}`, {
    method,
    headers: {
      "X-Authentication-Token": getZenHubAccessToken(),
      "Content-Type": "application/json",
    },
    body: isRead ? undefined : JSON.stringify(body),
  });
  const text = await response.text();
  if (!response.ok) {
    throw new Error(
      `Invalid status code: ${response.status} (${path} => ${text})`,
    );
  }
  return text === "" ? {} : JSON.parse(text);
};
