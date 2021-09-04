# gh-label-conductor
Small command line utility for updating the labels on a GitHub project from a YAML file. Used to quickly bootstrap common and consistent labels on a project.

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
cargo install gh-label-conductor
export -x GH_TOKEN <GITHUB_PERSONAL_ACCESS_TOKEN>
gh-label-conductor apply <owner>/<repo>
```

