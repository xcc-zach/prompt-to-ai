*AI GENERATED*

## Project Summary

**prompt-to-ai** (pai) is a Rust CLI tool that enhances developer workflows with LLM-powered automation. It provides four main commands: generating AI-assisted git commit messages, copying directory structures to clipboard, managing multiple LLM model configurations, and bootstrapping Claude/Codex environments by unpacking bundled assets (settings, skills, agents) to user home directories. The tool embeds static assets at compile time and supports configurable LLM endpoints for commit message generation.

## How to Read through the Codebase

1. Start with `src/main.rs` to understand the CLI structure and available commands (commit, ls, config, unpack)
2. Read `src/lib.rs` to see the module organization
3. Explore each command module in order of complexity:
   - `src/ls.rs` - Simple directory listing utility
   - `src/commit.rs` - LLM-powered commit message generation
   - `src/unpack.rs` - Asset extraction and environment setup
   - `src/config.rs` and `src/config/llm.rs` - Configuration management
4. Review utility modules under `src/utils/`:
   - `assets.rs` - Embedded asset access
   - `llm.rs` - LLM API client
   - `clip.rs` - Clipboard operations
   - `io.rs` - User input handling
5. Examine `build.rs` to understand how assets are embedded at compile time
6. Browse `src/assets/` to see bundled Claude configurations, skills, and agents

## Individual File Descriptions

- `src/main.rs`: CLI entrypoint using clap for argument parsing; defines Cli/Command enums for subcommand routing and dispatches to commit, ls, config, and unpack module functions with error handling.

- `src/lib.rs`: Library crate root that declares public modules (commit, config, ls, unpack) and private utils module, re-exporting utility functions at crate level.

- `src/commit.rs`: Git commit automation module using git2 and regex; analyzes working-tree changes via `get_change_str`, builds LLM prompts with `get_commit_prompt`, generates commit messages via `get_commit_message`, and stages/commits with `add_commit`.

- `src/unpack.rs`: Asset extraction module that deploys bundled files to `~/.claude` and `~/.codex` directories; includes `unpack_skills`, `unpack_claude_settings`, and `unpack_mcp` for optional GitHub MCP configuration with interactive PAT prompting.

- `src/ls.rs`: Directory structure utility defining `DirStructureFormat` enum (List/Tree variants) and recursive `dir_structure` function for generating formatted directory listings.

- `src/config.rs`: Configuration module root declaring llm (public), project (public, empty), and utils (private) submodules.

- `src/config/llm.rs`: LLM configuration management with `ModelConfigItem` struct and CRUD functions (`add_model_config`, `use_model_config`, `delete_model_config`, `get_model_configs`, `current_model_config`) persisting to `pai_config.toml` using BTreeMap for tag-based model selection.

- `src/config/utils.rs`: Error handling for config module defining `ConfigError` enum with variants for path, file I/O, and config read/write errors using thiserror.

- `src/utils.rs`: Utility module root declaring assets and llm (public) plus clip and io (private, re-exported) submodules.

- `src/utils/assets.rs`: Embedded asset access module defining `EmbeddedFile` struct and including build-generated `embedded_assets.rs`; provides `get_asset` for path-based lookup and `list_assets` iterator.

- `src/utils/llm.rs`: Synchronous LLM API client using reqwest; defines `Message` struct and `get_response`/`get_one_turn_response` functions for OpenAI-compatible chat completions with runtime model configuration.

- `src/utils/clip.rs`: Clipboard utility with `Clip` trait and fallback handling via `ClipFallback` enum; copies strings to system clipboard using arboard, falling back to print or temp file on headless systems.

- `src/utils/io.rs`: Simple I/O utility providing `prompt_to_input` function for interactive command-line user prompts.

- `build.rs`: Build script that recursively scans `src/assets/`, generates `embedded_assets.rs` with static ASSETS array using `include_bytes!` macros to embed files directly into the binary.

- `.github/workflows/rust-ci.yml`: GitHub Actions CI workflow running on ubuntu/windows/macos with stable Rust; executes tests, builds release binaries, and uploads platform-specific artifacts.

- `src/assets/claude/CLAUDE.md`: Bundled template for Claude AI guidelines specifying English comments and conventional commit message format with type prefixes.

- `src/assets/claude/settings.json`: Claude Code configuration with default model, plugins, permission rules, attribution templates, file suggestion command, and lifecycle hooks for codebase understanding automation.

- `src/assets/claude/file_suggestion.py`: Python fuzzy file/directory suggestion tool reading JSON query from stdin and outputting prioritized matches for Claude CLI autocomplete.

- `src/assets/claude/agents/file-summarizer.md`: Agent configuration for file-summarizer with purple theme, depending on summarize-file skill for generating file summaries.

- `src/assets/skills/ai-docs-commit/SKILL.md`: Skill for committing AI-generated documentation with standardized "ai docs:" message prefix.

- `src/assets/skills/summarize-file/SKILL.md`: Skill providing instructions and examples for generating concise, informative code file summaries.

- `src/assets/skills/read-codebase-docs/SKILL.md`: Skill to read `.aidocs` directory contents at session start for codebase understanding.

- `src/assets/skills/read-through-codebase/SKILL.md`: Skill for comprehensive codebase research and initial `.aidocs/codebase-overview.md` generation.

- `src/assets/skills/update-codebase-understanding/SKILL.md`: Skill to analyze git history, update file summaries for changed files, and refresh `.aidocs` documentation.

- `src/assets/skills/github-mcp-with-retry/SKILL.md`: Skill for retrying failed GitHub MCP tool calls up to 10 times for connection resilience.

- `src/assets/skills/ask_questions_if_underspecified/SKILL.md`: Skill defining workflow for detecting underspecified requests and asking clarifying questions before implementation.
