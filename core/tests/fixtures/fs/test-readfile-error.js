import { readFile } from 'node:fs/promises';

try {
    await readFile('./nonexistent-file-12345.txt');
    globalThis.testReadFileError = 'should_not_reach_here';
} catch (error) {
    globalThis.testReadFileError = 'error_caught';
}
