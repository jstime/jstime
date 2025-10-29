# Performance Regression Detection Workflow (Example)

This is an example GitHub Actions workflow for detecting performance regressions.
To enable it, rename this file to `.github/workflows/performance.yml`.

```yaml
name: Performance Benchmarks

on:
  pull_request:
    branches: [ main ]
  push:
    branches: [ main ]

jobs:
  benchmark:
    name: Run Performance Benchmarks
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0  # Need full history for comparisons

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Cache Rust dependencies
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/bin/
            ~/.cargo/registry/index/
            ~/.cargo/registry/cache/
            ~/.cargo/git/db/
            target/
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

      - name: Build release binary
        run: cargo build --release

      - name: Run JavaScript Benchmarks
        run: |
          ./target/release/jstime benchmarks/js_benchmarks.js > benchmark_results.txt
          cat benchmark_results.txt

      - name: Run Rust Benchmarks
        run: |
          # Install criterion if not cached
          cargo bench --bench runtime_bench -- --verbose

      - name: Upload benchmark results
        uses: actions/upload-artifact@v4
        with:
          name: benchmark-results
          path: |
            benchmark_results.txt
            target/criterion/

  # Optional: Compare with baseline
  compare:
    name: Compare with Baseline
    runs-on: ubuntu-latest
    if: github.event_name == 'pull_request'
    steps:
      - uses: actions/checkout@v4
        with:
          ref: ${{ github.base_ref }}

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Build and benchmark baseline
        run: |
          cargo bench --bench runtime_bench -- --save-baseline main

      - uses: actions/checkout@v4
        with:
          ref: ${{ github.head_ref }}
          clean: false

      - name: Build and benchmark PR
        run: |
          cargo bench --bench runtime_bench -- --baseline main

      - name: Comment PR with results
        if: github.event_name == 'pull_request'
        uses: actions/github-script@v7
        with:
          script: |
            const fs = require('fs');
            // Read criterion comparison output
            // Parse and format for GitHub comment
            // Post comment to PR
            github.rest.issues.createComment({
              issue_number: context.issue.number,
              owner: context.repo.owner,
              repo: context.repo.repo,
              body: 'Performance benchmark results will be added here'
            });
```

## Notes

- This workflow runs on every PR and push to main
- Criterion automatically detects performance regressions
- Results are uploaded as artifacts for review
- The compare job runs benchmarks on both baseline and PR branches
- Consider adding a threshold (e.g., >5% regression) to fail the workflow

## Customization

Adjust these settings based on your needs:
- `branches`: Which branches trigger the workflow
- Benchmark timeout values
- Regression thresholds
- Number of benchmark iterations
- Which benchmarks to run (can filter with `-- pattern`)
