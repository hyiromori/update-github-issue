import parser from "https://deno.land/x/yargs_parser@v20.2.7-deno/deno.ts";

const coreCommand = Deno.args[0] ?? "help";
const parsedArgs = parser(Deno.args.slice(1));

const toBoolean = (value: unknown, defaultValue: boolean): boolean =>
  typeof value === "boolean" ? value : defaultValue;
const toStringOrNull = (value: unknown): string | null =>
  ["string", "number"].includes(typeof value) ? `${value}` : null;
const toArray = (value: unknown): string[] => {
  if (Array.isArray(value)) {
    return value;
  } else if (typeof value === "string") {
    return [value];
  }
  return [];
};

const requireString = (value: unknown, valueName: string): string => {
  if (typeof value === "string") {
    return value;
  } else if (typeof value === "number") {
    return `${value}`;
  } else if (Array.isArray(value)) {
    throw new Error(`${valueName} が複数回指定されています`);
  }
  throw new Error(`${valueName} を指定してください`);
};
const requireArray = (value: string[] | null, valueName: string): string[] => {
  if (Array.isArray(value) && value.length > 0) {
    return value;
  }
  throw new Error(`${valueName} を指定してください`);
};
const requireGitHubUrl = (value: string | null, valueName: string): string => {
  if (typeof value === "string" && value.startsWith("https://github.com/")) {
    return value;
  }
  throw new Error(`${valueName} には GitHub の URL を指定してください`);
};

interface Args {
  epicUrl: string | null;
  labels: string[] | null;
  owner: string | null;
  pipeline: string | null;
  repository: string | null;
  title: string | null;
  urls: string[];
  verbose: boolean;
  workspace: string | null;
}

const args: Args = {
  epicUrl: toStringOrNull(parsedArgs.epicUrl),
  labels: toArray(parsedArgs.label),
  owner: toStringOrNull(parsedArgs.owner),
  pipeline: toStringOrNull(parsedArgs.pipeline),
  repository: toStringOrNull(parsedArgs.repository),
  title: toStringOrNull(parsedArgs.title),
  urls: toArray(parsedArgs._).map((url) => requireGitHubUrl(url, "---urls")),
  verbose: toBoolean(parsedArgs.verbose, false),
  workspace: toStringOrNull(parsedArgs.workspace),
};

export const getCoreCommand = (): string => coreCommand;
export const verbose = (): boolean => args.verbose;

export const getOwner = (): string => requireString(args.owner, "--owner");
export const getRepository = (): string =>
  requireString(args.repository, "--repository");
export const getTitle = (): string => requireString(args.title, "--title");
export const getUrls = (): string[] => requireArray(args.urls, "--url");
export const getWorkspace = (): string =>
  requireString(args.workspace, "--string");

export const getLabels = (): string[] | null => args.labels;
export const getPipeline = (): string | null => args.pipeline;
export const getEpicUrl = (): string | null => args.epicUrl;
