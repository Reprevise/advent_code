import fs from "fs";
import { performance } from "perf_hooks";

try {
  const t0 = performance.now();
  const data = fs.readFileSync("input.txt", "utf-8");
  const passwords = data.split("\n");
  getAnswer(passwords);
  const t1 = performance.now();
  console.log(`In total, this calculation took ${t1 - t0}`);
} catch (e) {
  console.error(e);
}

function getAnswer(input: string[]) {
  const t0 = performance.now();
  let passes = 0;
  for (const data of input) {
    const workable = data.split(" ");
    const positions = workable[0].split("-");
    const posOne: number = +positions[0],
      posTwo: number = +positions[1];
    const char = workable[1][0];
    const password = workable[2];
    const charPosOne = password[posOne - 1];
    const charPosTwo = password[posTwo - 1];
    if (charPosOne === charPosTwo) continue;
    if (charPosOne === char || charPosTwo === char) passes++;
  }
  const t1 = performance.now();
  console.log(passes);
  console.log(`That took ${t1 - t0}ms!`);
}
