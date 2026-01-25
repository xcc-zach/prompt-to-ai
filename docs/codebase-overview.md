*AI GENERATED*

## Project Summary

This is a Rust CLI tool called "pai" (prompt-to-ai) that assists developers with LLM-powered workflows. The main features include:

1. **Automated Git Commit Messages**: Analyzes git diff and generates conventional commit messages using LLM APIs
2. **Directory Structure Listing**: Outputs directory contents to clipboard in tree or list format
3. **LLM Configuration Management**: Manages multiple LLM model configurations (API keys, endpoints, model names)
4. **Asset Unpacking**: Bootstraps Claude Code/Codex environments by extracting bundled skills and configurations to user home directories

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

- `src/main.rs`: CLI entrypoint that uses clap for argument parsing, defines the command structure (Commit, Ls, Config, Unpack), and dispatches to appropriate module functions for LLM-assisted commit workflow, directory listing, configuration management, and asset unpacking.

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
