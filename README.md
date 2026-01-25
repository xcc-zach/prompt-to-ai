# prompt-to-ai
Convert directories to AI readable text, and use AI to process that text.

# Download

[Release](https://github.com/xcc-zach/prompt-to-ai/releases/tag/v0.1.1)

# Usage

(Optional) add LLM:
```bash
pai config add-model
```

Generate commit message:
```bash
pai commit
```

Generate commit message automatically with LLM and commit:
```bash
pai commit --auto
```

List current directory:
```bash
pai ls
```

For help:
```
pai --help
pai <subcommand> --help
```
