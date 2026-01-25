*AI GENERATED*

## Project Summary

This is a Rust CLI tool called "pai" (prompt-to-ai) that assists developers with LLM-powered workflows. The main features include:

1. **Automated Git Commit Messages**: Analyzes git diff and generates conventional commit messages using LLM APIs
2. **Directory Structure Listing**: Outputs directory contents to clipboard in tree or list format
3. **LLM Configuration Management**: Manages multiple LLM model configurations (API keys, endpoints, model names)
4. **Asset Unpacking**: Bootstraps Claude Code/Codex environments by extracting bundled skills, agents, and configurations to user home directories

The tool uses an OpenAI-compatible API interface, allowing it to work with various LLM providers.

## How to Read Through the Codebase

1. Start with `src/main.rs` to understand the CLI structure, available commands, and how they dispatch to different modules
2. Read `src/lib.rs` to see the module organization and public API exports
3. For commit functionality: `src/commit.rs` -> `src/utils/llm.rs` -> `src/config/llm.rs`
4. For configuration: `src/config.rs` -> `src/config/llm.rs` -> `src/config/utils.rs`
5. For unpacking: `src/unpack.rs` -> `src/utils/assets.rs`
6. For utilities: `src/utils.rs` -> individual utility modules (`clip.rs`, `io.rs`)
7. Review `.github/workflows/rust-ci.yml` to understand the CI/CD pipeline

## Individual File Descriptions

- `src/main.rs`: CLI entrypoint that uses clap for argument parsing with comprehensive help messages, defines the command structure (Commit, Ls, Config, Unpack) with descriptive help text for all subcommands and arguments, and dispatches to appropriate module functions for LLM-assisted commit workflow, directory listing, configuration management, and asset unpacking.

- `src/lib.rs`: Library root that declares four public submodules (commit, config, ls, unpack) and one private submodule (utils), re-exporting utility functions for external consumers.

- `src/commit.rs`: Automates git commit message generation by analyzing working directory changes using git2, building LLM prompts with diff information, and calling the LLM API to produce conventional commit messages (feature/fix/refactor/docs/chore format).

- `src/unpack.rs`: Bootstraps local Claude/Codex environments by extracting bundled embedded assets to ~/.claude and ~/.codex directories, and optionally configures GitHub MCP integration via interactive PAT prompt.

- `src/ls.rs`: Utility module that recursively traverses directories and returns contents as formatted strings (either indented tree with filenames or flat list with full paths).

- `src/config.rs`: Parent configuration module that organizes submodules for LLM-related and project-related configuration logic.

- `src/config/llm.rs`: Manages LLM model configurations by persisting them to pai_config.toml, allowing users to register multiple named model configurations (API key, model name, base URL), switch between them, and retrieve the active configuration.

- `src/config/utils.rs`: Defines ConfigError enum with five variants for structured error handling across configuration operations, using thiserror for trait implementations.

- `src/config/project.rs`: Empty placeholder module reserved for future project-level configuration logic.

- `src/utils.rs`: Module organizer that declares and re-exports submodules for assets, LLM, clipboard, and I/O utilities.

- `src/utils/llm.rs`: Synchronous HTTP client wrapper for LLM chat completions using OpenAI-compatible API format, supporting both single-turn and multi-turn conversations.

- `src/utils/clip.rs`: Clipboard abstraction layer using arboard, with fallback to printing or saving to temporary file when clipboard is unavailable.

- `src/utils/io.rs`: Simple interactive input utility providing prompt_to_input function for collecting user input from stdin.

- `src/utils/assets.rs`: Runtime interface for accessing files bundled into the binary at compile time, providing functions to look up and iterate over embedded assets.

- `.github/workflows/rust-ci.yml`: GitHub Actions CI workflow that runs on Ubuntu/Windows/macOS, performs format checking, runs tests, builds release binaries, and uploads platform-specific artifacts on pushes to main/master and pull requests.

## Embedded Assets

The following files are bundled into the binary at compile time and extracted via the `unpack` command to set up Claude Code and Codex environments:

### Claude Code Agents

- `src/assets/claude/agents/file-summarizer.md`: YAML-frontmatter Markdown file that defines a Claude Code agent named "file-summarizer" with purple color theme, declares a single skill dependency ("summarize-file"), and instructs the agent to use that skill to summarize files according to the detailed requirements defined within the skill itself.

### Claude Code Configuration

- `src/assets/claude/CLAUDE.md`: Global instructions file that enforces English for code comments and commit messages, and establishes conventional commit message format with type prefixes (feat, fix, docs, style, refactor, perf, test) followed by a colon and concise description.

- `src/assets/claude/settings.json`: Claude Code settings configuration that specifies the model (opus), enables rust-analyzer-lsp plugin, sets permission rules for file access and bash commands, defines attribution templates for commits and PRs, configures a file suggestion command, includes a PostToolUse hook that triggers after git commits to remind about updating codebase documentation for non-document changes, and includes a SessionStart hook that reminds to trigger the read-codebase-docs skill at the beginning of new sessions.

### Claude Code Skills

- `src/assets/skills/read-through-codebase/SKILL.md`: YAML-frontmatter skill definition that depends on the `ai-docs-commit` skill for committing and references an external instruction template at `instructions/codebase-overview.md`; it specifies a procedural workflow that directs Claude Code to analyze a codebase comprehensively, write a `docs/codebase-overview.md` file (skipping if it already exists), and then invoke the ai-docs-commit skill to commit.

- `src/assets/skills/read-through-codebase/instructions/codebase-overview.md`: Markdown instruction template that directs an AI agent to enumerate all relevant source files in a project (excluding docs, configs, and build artifacts), invoke `file-summarizer` agents or the `summarize-file` skill to generate per-file summaries, then produce a consolidated overview comprising a project summary, a recommended reading order, and individual file descriptions.

- `src/assets/skills/ai-docs-commit/SKILL.md`: YAML-frontmatter skill definition with fork context that instructs the agent to run `git add` followed by `git commit -m "ai docs: $ARGUMENTS"` for committing AI-generated or modified documentation files, explicitly ensuring only documentation files are staged and providing a standardized commit message format for AI documentation changes.

- `src/assets/skills/update-codebase-understanding/SKILL.md`: Skill configuration that depends on the `read-through-codebase`, `summarize-file`, and `ai-docs-commit` skills; it defines a workflow that fetches git history to find commits after the last "ai docs:" commit, analyzes code changes across those commits, uses `summarize-file` to get updated summaries for changed files, updates documents under `docs/` accordingly, and commits with the `ai-docs-commit` skill.

- `src/assets/skills/read-codebase-docs/SKILL.md`: Skill definition that instructs the agent to read all documents under the `docs` directory that were created via the `read-through-codebase` skill at session start, outputting a message if those documents do not exist but without triggering the `read-through-codebase` skill automatically.
