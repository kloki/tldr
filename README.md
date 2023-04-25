# 👮 tldr-line-verifier

Command line tool to verify the max length of files in a repository. Supposed to be run in CI.

## Install

```
cargo install tldr-line-verifier
```

## Run

```
tldr-line-verifier ./ --max_lines=1000 --exclude_pattern=".csv$"

```

# Docker

```
docker run -v /path/to/repository:/home nicekloki/tldr-line-verifier
```

# Github action

```
jobs:
  tldr:
    runs-on: ubuntu-latest
    container:
      image: nicekloki/tldr-line-verifier
    steps:
      - uses: actions/checkout@v3
      - name: check file length
        run: tldr-line-verifier ./ --max-lines=1000
```
