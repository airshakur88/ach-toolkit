import { Buffer } from "node:buffer";

const [, , cmd, ...rest] = process.argv;

function slugify(s: string): string {
  return s
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");
}

switch (cmd) {
  case "slugify":
    console.log(slugify(rest.join(" ")));
    break;
  case "b64encode":
    console.log(Buffer.from(rest.join(" ")).toString("base64"));
    break;
  case "b64decode":
    console.log(Buffer.from(rest.join(" "), "base64").toString("utf8"));
    break;
  default:
    console.error("usage: bun run ts/cli.ts <slugify|b64encode|b64decode> [input]");
    process.exit(1);
}