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
    const minMax = workable[0].split("-");
    const min: number = +minMax[0],
      max: number = +minMax[1];
    const char = workable[1][0];
    const password = workable[2];
    const expression = new RegExp(`[^${char}]`, "g");
    const times = password.replace(expression, "").length;
    if (times >= min && times <= max) {
      passes++;
    }
  }
  const t1 = performance.now();
  console.log(passes);
  console.log(`That took ${t1 - t0}ms!`);
}
