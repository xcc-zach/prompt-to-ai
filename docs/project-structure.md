*AI GENERATED*

Project Summary:
This is a Rust CLI tool called `pai` (prompt-to-ai) that provides developer productivity utilities centered around AI/LLM integration. It offers four main capabilities: (1) AI-powered git commit message generation that analyzes working tree changes and uses an LLM to generate appropriate commit messages, (2) directory structure listing with clipboard integration, (3) LLM model configuration management for storing and switching between different API endpoints, and (4) an asset unpacking system that bootstraps Claude Code and Codex environments by deploying bundled skills, settings, and configuration files to user directories. The project embeds asset files at compile time for portable distribution.

Individual File Descriptions:
- `src\main.rs`: Defines the CLI entry point using clap for argument parsing, orchestrating four subcommands (commit, ls, config, unpack) by delegating to library modules and providing clipboard integration via the ClipFallback trait.
- `src\lib.rs`: Serves as the crate root module, declaring four public submodules (commit, config, ls, unpack) and one private submodule (utils), re-exporting utility items for a unified API surface.
- `src\commit.rs`: Provides git commit functionality including `get_change_str` for analyzing working tree changes via git2, `get_commit_prompt` for constructing LLM prompts, `get_commit_message` for generating AI commit messages, and `add_commit` for staging and committing changes.
- `src\ls.rs`: Defines `DirStructureFormat` enum (List/Tree) and the recursive `dir_structure` function for generating directory structure representations as either flat path lists or indented tree views.
- `src\config.rs`: Module aggregator declaring three submodules (llm, project, utils) to organize configuration-related functionality.
- `src\config\llm.rs`: Defines `ModelConfigItem`, `ModelConfig`, and `Config` structs for LLM configuration, providing CRUD operations (add, use, delete, get, current) to manage multiple tagged LLM endpoint configurations persisted as TOML.
- `src\config\project.rs`: Placeholder module file (currently empty) in the config module structure.
- `src\config\utils.rs`: Defines `ConfigError` enum with five error variants (PathError, FileReadError, FileWriteError, ConfigWriteError, ConfigReadError) for configuration-related error handling.
- `src\utils.rs`: Module aggregator declaring four submodules (assets, clip, io, llm) and re-exporting clipboard and I/O utilities.
- `src\utils\clip.rs`: Defines `Clip` trait, `ClipFallback` enum (Print/Save), and `FileHandle` struct for cross-platform clipboard operations with graceful degradation to stdout printing or temp file writing when clipboard is unavailable.
- `src\utils\io.rs`: Provides `prompt_to_input` helper function for interactive console input with prompt display and whitespace trimming.
- `src\utils\llm.rs`: Defines `LLMError`, `Message` struct, and request/response DTOs for OpenAI-compatible API calls, exposing `get_one_turn_response` and `get_response` functions for synchronous LLM chat completions.
- `src\utils\assets.rs`: Defines `EmbeddedFile` struct and includes a build-generated `ASSETS` array, providing `get_asset` and `list_assets` functions for compile-time embedded asset lookup and enumeration.
- `src\unpack.rs`: Provides asset extraction functions (`list_by_prefix`, `unpack_skills`, `unpack_claude_settings`, `unpack_mcp`, `unpack_all`) to deploy bundled skills to ~/.claude/skills and ~/.codex/skills, settings to ~/.claude, and interactively configure GitHub MCP integration.
- `build.rs`: Build script that recursively collects files from `src/assets` and generates `embedded_assets.rs` with a static ASSETS array using `include_bytes!` to embed assets at compile time.
- `.github\workflows\rust-ci.yml`: GitHub Actions CI workflow that runs formatting checks, tests, and release builds on Ubuntu/Windows/macOS, uploading the `pai` executable as an artifact on pushes to main/master and on pull requests.
- `src\assets\claude\CLAUDE.md`: Bundled configuration directive instructing Claude to use English for code comments and commit messages.
- `src\assets\claude\file_suggestion.py`: Python script providing fuzzy file/directory suggestion for Claude Code, prioritizing source files and exact basename matches.
- `src\assets\claude\settings.json`: JSON configuration template for Claude Code IDE settings including model preference, plugins, permissions, and file suggestion hook.
- `src\assets\skills\github-mcp-with-retry\SKILL.md`: Claude skill that instructs retry of GitHub MCP tool calls up to 10 turns on failure for resilience against server instability.
- `src\assets\skills\read-through-codebase\SKILL.md`: Claude skill for comprehensively researching and documenting codebases by generating project structure documentation.
- `src\assets\skills\read-through-codebase\instructions\commit.md`: Instruction file specifying the standardized commit message for codebase documentation.
- `src\assets\skills\read-through-codebase\instructions\project-structure.md`: Instruction template directing AI to enumerate and summarize all source files in a project.
- `src\assets\skills\summarize-file\SKILL.md`: Claude skill for generating concise one-sentence file summaries covering functions, classes, and overall purpose.
