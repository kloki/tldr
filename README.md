# 👮 tldr-line-verifier

Command line tool to verify the max length of files in a repository. Supposed to be run in CI.

## Binaries

Check [Releases](https://github.com/kloki/tldr/releases) for binaries and installers

## Run

```
tldr-line-verifier ./ --max_lines=1000 --exclude_pattern=".csv$"

```

# Enforce agent context file size

AI coding agents use context files like `CLAUDE.md` and `AGENTS.md` to stay on track. When these files grow too large, agents lose focus and performance degrades. Use `tldr-line-verifier` to enforce a maximum size on these files:

```
tldr-line-verifier ./ --max-lines=300 --include-pattern="(CLAUDE|AGENTS)\\.md$"
```

# Github action

To enforce agent context file size in CI:

```
jobs:
  agent-context-check:
    runs-on: ubuntu-latest
    container:
      image: nicekloki/tldr-line-verifier:latest
    steps:
      - uses: actions/checkout@v3
      - name: check agent context files
        run: tldr-line-verifier ./ --max-lines=300 --include-pattern="(CLAUDE|AGENTS)\\.md$"
```
