# Test Fixtures Organization

This directory contains test fixture files organized by feature area.

## Directory Structure

- **`fs/`** - File system related fixtures for testing the FS API
  - Test files for various file operations (read, write, copy, etc.)
  - Sample directories for testing directory operations

- **`modules/`** - JavaScript module fixtures for testing module loading
  - `hello-world.js` - Basic module export test
  - `import-meta-example.js` - Module metadata tests

- **`json/`** - JSON module fixtures for testing JSON imports
  - `simple.json` - Simple JSON structure
  - `test-data.json` - Complex JSON structure with nested objects

## Adding New Fixtures

When adding new test fixtures:

1. Place them in the appropriate feature directory
2. If no suitable directory exists, create a new one with a descriptive name
3. Use clear, descriptive filenames that indicate what the fixture tests
4. Keep fixtures minimal and focused on testing specific functionality

## Guidelines

- Keep fixture files small and focused
- Use descriptive names that clearly indicate the test purpose
- Group related fixtures together in subdirectories
- Document any complex or non-obvious fixtures
