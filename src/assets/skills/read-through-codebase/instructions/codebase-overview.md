Describe what each file does under this project. First list all relevant files. For the brief description of each file, use `file-summarizer` agent to summarize that file, or use `summarize-file` skill with argument `path/to/file` as fallback. If you use agents, you can start many agents in parallel for the files to describe. Ignore `README.md`, `LICENSE`, `.gitignore`, `.env`, project config files like `requirements.txt` or `Cargo.toml`, files in `docs/` and build artifacts. The files should be listed in the order of importance. After listing all files, provide a short summary of the overall project purpose and functionality. Then provide instructions on how to read through the codebase effectively (start from which file, then to which file/directory, then to ...).

Output example:
```markdown
*AI GENERATED*
Project Summary:
<overall project summary>
How to Read through the Codebase:
<instructions on how to read through the codebase>
Individual File Descriptions:
- `src\main.rs`:  <description>
- `src\unpack.rs`: <description>
- `.github\workflows\rust-ci.yml`: <description>
```