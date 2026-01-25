# prompt-to-ai

A CLI tool for LLM-powered developer workflows.

## Features

- **Automated Git Commits**: Generate conventional commit messages using LLM
- **Directory Listing**: Output directory contents to clipboard
- **LLM Configuration**: Manage multiple model configs (API keys, endpoints)
- **Asset Unpacking**: Bootstrap Claude Code/Codex environments

## Download

[Release](https://github.com/xcc-zach/prompt-to-ai/releases/tag/v0.1.1)

## Usage

### Commit

Generate commit message (copies prompt to clipboard):
```bash
pai commit
```

Auto-generate and commit with LLM:
```bash
pai commit --auto
```

Force English commit message:
```bash
pai commit -e
pai commit --auto -e
```

### List Directory

Copy directory structure to clipboard:
```bash
pai ls
```

### LLM Configuration

Add a model:
```bash
pai config add-model
```

Switch active model:
```bash
pai config use-model
```

List all models:
```bash
pai config list-models
```

Remove a model:
```bash
pai config remove-model
```

### Unpack (Claude Code/Codex Setup)

Extract bundled assets to set up Claude Code and Codex environments:
```bash
pai unpack
```

This command:
1. Extracts skills, agents, and configurations to `~/.claude/`
2. Sets up Codex environment in `~/.codex/`
3. Optionally configures GitHub MCP integration (prompts for PAT)

#### Included Assets

**Skills:**
- `read-through-codebase` - Analyze codebase and generate documentation
- `read-codebase-docs` - Read AI-generated docs at session start
- `update-codebase-understanding` - Update docs after code changes
- `summarize-file` - Generate file summaries
- `ai-docs-commit` - Commit AI documentation with standard format
- `github-mcp-with-retry` - Retry GitHub MCP calls on failure

**Agents:**
- `file-summarizer` - Parallel file summarization

**Configuration:**
- `CLAUDE.md` - Global instructions (commit conventions)
- `settings.json` - Model, permissions, hooks

## Help

```bash
pai --help
pai <subcommand> --help
```
