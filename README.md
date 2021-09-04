# gh-label-conductor
Small command line utility for updating the labels on a GitHub project via a config file. Used to quickly bootstrap common and consistent labels on a project.

## Usage
```fish
export -x GH_TOKEN <GITHUB_PERSONAL_ACCESS_TOKEN>
cargo run -- apply <owner>/<repo>
```
