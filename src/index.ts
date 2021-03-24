import { getCoreCommand } from "./lib/args.ts";
import { update } from "./command/update.ts";
import { create } from "./command/create.ts";
import { help } from "./command/help.ts";

switch (getCoreCommand()) {
  case "create":
    await create();
    break;
  case "update":
    await update();
    break;
  default:
    help();
}
