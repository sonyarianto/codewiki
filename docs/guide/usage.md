# Commands

## Quick Start

```bash
# Step 1: Configure your LLM provider
wakawiki --init

# Step 2: Generate documentation (interactive)
wakawiki

# Or: One-shot non-interactive
wakawiki -p "Summarize the architecture of this project"

# Or: Update existing documentation
wakawiki --update
```

## CLI Reference

```
wakawiki [OPTIONS] [PROMPT]
```

| Option | Description |
|--------|-------------|
| `--init` | Interactive setup: choose provider, set API key, pick model |
| `-p`, `--print` | Non-interactive one-shot mode (CI-friendly) |
| `--update` | Refresh existing `wakawiki/` docs with incremental diff |
| `-h`, `--help` | Show help |

## Interactive Mode

Running `wakawiki` without `-p` starts an interactive chat session with the LLM agent. You can guide the documentation process by asking questions or giving instructions.

```bash
wakawiki
# > Please focus on the API layer
# > Add more detail about error handling
# > Generate mermaid diagrams for the architecture
```

## One-Shot Mode

Use `-p` / `--print` for non-interactive, single-prompt runs. This is ideal for scripts and CI pipelines.

```bash
wakawiki -p "Document the public API surface"
```

## Incremental Updates

Once you have an existing `wakawiki/` directory, run `--update` to refresh only the files that changed since the last run. This avoids regenerating everything from scratch.

```bash
wakawiki --update
```
