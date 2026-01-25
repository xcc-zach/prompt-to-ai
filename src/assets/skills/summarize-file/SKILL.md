---
name: summarize-file
description: Get the informative summary of a file, especially for code files. Use when needing to understand the purpose and functionality of a specific file.
context: fork
---

Summarized the file $ARGUMENTS. The output summary should include dependencies of the file; what functions, classes and structs are defined within the file; interactions between them; the overall function and intention of the file; and try the best to summarize those in one concise sentence.

e.g.
1.`src\utils.rs`:  This module file declares submodules assets (public) and llm (public) plus private submodules clip and io, and then re-exports everything from clip and io (pub use clip::*; pub use io::*;) so callers can access their APIs directly through this parent module while keeping the internal module layout organized.
2.`src\unpack.rs`: This Rust file depends on crate::utils::assets, defines no new structs/classes, but provides helper functions list_by_prefix, unpack_skills, unpack_claude_settings, unpack_mcp, and the public entrypoint unpack_all; it filters embedded assets (EmbeddedFile) via list_assets(), writes skill and Claude config files into ~/.claude and ~/.codex (deleting any conflicting existing paths first), then interactively prompts for a GitHub PAT to run claude mcp add-json to register a GitHub MCP, with unpack_all orchestrating these steps to bootstrap a user’s local Claude/Codex environment from bundled assets.
3.`.github\workflows\rust-ci.yml`: This GitHub Actions workflow defines a single matrix job build-test that runs on Ubuntu/Windows/macOS with stable Rust, checks out the repo, installs Rust with rustfmt/clippy, caches Cargo builds, enforces formatting on Ubuntu, runs cargo test --all-features --locked, builds a release binary, and uploads the OS-specific pai executable as an artifact, aiming to continuously validate and package the project on pushes to main/master and on all pull requests.