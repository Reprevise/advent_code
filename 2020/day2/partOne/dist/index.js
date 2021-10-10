"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const fs_1 = __importDefault(require("fs"));
const perf_hooks_1 = require("perf_hooks");
try {
    const t0 = perf_hooks_1.performance.now();
    const data = fs_1.default.readFileSync("input.txt", "utf-8");
    const passwords = data.split("\n");
    getAnswer(passwords);
    const t1 = perf_hooks_1.performance.now();
    console.log(`In total, this calculation took ${t1 - t0}`);
}
catch (e) {
    console.error(e);
}
function getAnswer(input) {
    const t0 = perf_hooks_1.performance.now();
    let passes = 0;
    for (const data of input) {
        const workable = data.split(" ");
        const minMax = workable[0].split("-");
        const min = +minMax[0], max = +minMax[1];
        const char = workable[1][0];
        const password = workable[2];
        const expression = new RegExp(`[^${char}]`, "g");
        const times = password.replace(expression, "").length;
        if (times >= min && times <= max) {
            passes++;
        }
    }
    const t1 = perf_hooks_1.performance.now();
    console.log(passes);
    console.log(`That took ${t1 - t0}ms!`);
}
//# sourceMappingURL=index.js.map