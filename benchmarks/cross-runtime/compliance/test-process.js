// Compliance test for Process API
// Tests process.env, process.argv, process.cwd, and process.exit

let passed = 0;
let failed = 0;

function test(name, fn) {
  try {
    fn();
    passed++;
  } catch (e) {
    console.error(`FAIL: ${name} - ${e.message}`);
    failed++;
  }
}

// Test process object
test('process exists', () => {
  if (typeof process !== 'object') throw new Error('process is not an object');
});

// Test process.env
test('process.env exists', () => {
  if (typeof process.env !== 'object') throw new Error('process.env is not an object');
});

test('process.env contains PATH', () => {
  // PATH should exist on all systems (Unix, Windows has Path or PATH)
  const hasPath = 'PATH' in process.env || 'Path' in process.env;
  if (!hasPath) throw new Error('process.env does not contain PATH');
});

test('process.env property access', () => {
  // Should be able to access existing env vars
  const path = process.env.PATH || process.env.Path;
  if (typeof path !== 'string') throw new Error('process.env.PATH is not a string');
  if (path.length === 0) throw new Error('process.env.PATH is empty');
});

// Test process.argv
test('process.argv exists', () => {
  if (!Array.isArray(process.argv)) throw new Error('process.argv is not an array');
});

test('process.argv has executable', () => {
  if (process.argv.length < 1) throw new Error('process.argv should have at least one element');
  if (typeof process.argv[0] !== 'string') throw new Error('process.argv[0] is not a string');
});

// Test process.cwd
test('process.cwd exists', () => {
  if (typeof process.cwd !== 'function') throw new Error('process.cwd is not a function');
});

test('process.cwd returns string', () => {
  const cwd = process.cwd();
  if (typeof cwd !== 'string') throw new Error('process.cwd() did not return a string');
  if (cwd.length === 0) throw new Error('process.cwd() returned empty string');
});

test('process.cwd returns absolute path', () => {
  const cwd = process.cwd();
  // On Unix-like systems, absolute paths start with /
  // On Windows, they start with a drive letter like C:
  const isAbsolute = cwd.startsWith('/') || /^[A-Za-z]:/.test(cwd);
  if (!isAbsolute) throw new Error('process.cwd() did not return an absolute path');
});

// Test process.exit
test('process.exit exists', () => {
  if (typeof process.exit !== 'function') throw new Error('process.exit is not a function');
});

// Test process.stdout
test('process.stdout exists', () => {
  if (typeof process.stdout !== 'object') throw new Error('process.stdout is not an object');
});

test('process.stdout.write exists', () => {
  if (typeof process.stdout.write !== 'function') throw new Error('process.stdout.write is not a function');
});

// Test process.stderr
test('process.stderr exists', () => {
  if (typeof process.stderr !== 'object') throw new Error('process.stderr is not an object');
});

test('process.stderr.write exists', () => {
  if (typeof process.stderr.write !== 'function') throw new Error('process.stderr.write is not a function');
});

// Test process.stdin
test('process.stdin exists', () => {
  if (typeof process.stdin !== 'object') throw new Error('process.stdin is not an object');
});

// Report results
console.log(`Process API: ${passed} passed, ${failed} failed`);
if (failed > 0) process.exit(1);
