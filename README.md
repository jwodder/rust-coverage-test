This repository is used as a testing ground for trying out various combinations
of test coverage programs and coverage-viewing websites for use with Rust code.
The repository contains a simple library with tests, and each non-default
branch has a GitHub Actions workflow that prepares a coverage report using some
tool and submits a report to either Codecov or Coveralls.  If things go
perfectly, the online coverage view should show 100% coverage.  For some
reason, things do not go well for most coverage tools.

Branches
========

| Branch | Workflow | Coverage Tool | Coverage View (Reported Coverage) |
| ------ | -------- | ------------- | --------------------------------- |
| grcov-codecov | [[link]](https://github.com/jwodder/rust-coverage-test/blob/grcov-codecov/.github/workflows/test.yml) | [grcov](https://github.com/mozilla/grcov) | [Codecov](https://app.codecov.io/gh/jwodder/rust-coverage-test/branch/grcov-codecov) (2.21%) |
| grcov-coveralls | [[link]](https://github.com/jwodder/rust-coverage-test/blob/grcov-coveralls/.github/workflows/test.yml) | [grcov](https://github.com/mozilla/grcov) | [Coveralls](https://coveralls.io/github/jwodder/rust-coverage-test?branch=grcov-coveralls) (0%) |
| tarpaulin-codecov | [[link]](https://github.com/jwodder/rust-coverage-test/blob/tarpaulin-codecov/.github/workflows/test.yml) | [tarpaulin](https://github.com/xd009642/tarpaulin) | [Codecov](https://app.codecov.io/gh/jwodder/rust-coverage-test/branch/tarpaulin-codecov) (27.27%) |
| tarpaulin-coveralls | [[link]](https://github.com/jwodder/rust-coverage-test/blob/tarpaulin-coveralls/.github/workflows/test.yml) | [tarpaulin](https://github.com/xd009642/tarpaulin) | [Coveralls](https://coveralls.io/github/jwodder/rust-coverage-test?branch=tarpaulin-coveralls) (83.33% — everything except doctest coverage) |
| llvm-cov-codecov | [[link]](https://github.com/jwodder/rust-coverage-test/blob/llvm-cov-codecov/.github/workflows/test.yml) | [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) | [Codecov](https://app.codecov.io/gh/jwodder/rust-coverage-test/branch/llvm-cov-codecov) (81.25% — everything except doctest coverage) |
