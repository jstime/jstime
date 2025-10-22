import { readdir } from 'node:fs/promises';

try {
    await readdir('./nonexistent-dir-12345');
    globalThis.testReadDirError = 'should_not_reach_here';
} catch (error) {
    globalThis.testReadDirError = 'error_caught';
}
