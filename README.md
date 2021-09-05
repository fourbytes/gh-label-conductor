# gh-label-conductor

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]
[![Coverage][codecov-badge]][codecov-url]

[crates-badge]: https://img.shields.io/crates/v/gh-label-conductor.svg
[crates-url]: https://crates.io/crates/gh-label-conductor
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE
[actions-badge]: https://github.com/fourbytes/gh-label-conductor/actions/workflows/tests.yml/badge.svg
[actions-url]: https://github.com/fourbytes/gh-label-conductor/actions/workflows/tests.yml
[codecov-badge]: https://codecov.io/gh/fourbytes/gh-label-conductor/branch/main/graph/badge.svg
[codecov-url]: https://codecov.io/gh/fourbytes/gh-label-conductor

Small command line utility for updating the labels on a GitHub project from a YAML file. Used to quickly bootstrap common and consistent labels on a project.

> **Note**: Requires Rust nightly due to an [upstream dependency](https://github.com/oxidecomputer/third-party-api-clients/tree/main/github).

## Example Source
```yaml
categories:
  - prefix: P
    color: "D93F0B"
    labels:
      low: "Priority: Low"
      medium: "Priority: Medium"
      high: "Priority: High"
      critical: "Priority: Critical"
  - prefix: C
    color: "BFD4F2"
    labels:
      bug: "Category: This is a bug."
      enhancement: "Category: An issue or PR with for a proposed enhancement."
      tracking-issue: "Category: A tracking issue for a major feature or change."
```

## Usage
```fish
cargo +nightly install gh-label-conductor
export -x GH_TOKEN <GITHUB_PERSONAL_ACCESS_TOKEN>
gh-label-conductor apply <owner>/<repo>
```

