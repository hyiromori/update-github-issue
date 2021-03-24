import { verbose } from "./args.ts";

export const infoLog = (log: string) => {
  console.log("INFO: %s", log);
};

export const debugLog = (log: string) => {
  if (verbose()) {
    console.log("DEBUG: %s", log);
  }
};
