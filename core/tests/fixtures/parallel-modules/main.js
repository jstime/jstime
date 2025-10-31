// Main entry point - depends on lib-x, lib-y, and util-c
import { libX } from './lib-x.js';
import { libY } from './lib-y.js';
import { processC } from './util-c.js';

const result = {
    x: libX(5),      // processA(processB(5)) = (5+10)*2 = 30
    y: libY(10),     // processB(processC(10)) = (10-5)+10 = 15
    c: processC(20)  // 20-5 = 15
};

globalThis.parallelTestResult = result;
