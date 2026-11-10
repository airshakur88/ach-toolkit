import { Buffer } from "node:buffer";

export function b64encode(s: string): string {
  return Buffer.from(s, "utf8").toString("base64");
}

export function b64decode(b: string): string {
  return Buffer.from(b, "base64").toString("utf8");
}

const [cmd, ...rest] = process.argv.slice(2);
const data = rest.join(" ");
if (cmd === "b64encode") console.log(b64encode(data));
else if (cmd === "b64decode") console.log(b64decode(data));
else {
  console.error("usage: bun base64.ts <b64encode|b64decode> [input]");
  process.exit(1);
}