import { readFileSync, writeFileSync } from "fs";
import { join } from "path";

async function main() {
  const fpath = join("mocks", "program.info");
  const pInfo = JSON.parse(readFileSync(fpath).toString()).account.data;
  console.log(pInfo);
}

main();
