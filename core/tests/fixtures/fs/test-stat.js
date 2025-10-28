import { stat } from 'node:fs/promises';

const stats = await stat('./tests/fixtures/fs/test-readfile.txt');
globalThis.testStat = stats.isFile === true && stats.size > 0;
