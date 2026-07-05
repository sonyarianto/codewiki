# What is WakaWiki?

WakaWiki is a CLI tool that uses LLMs to automatically generate and maintain structured documentation for any codebase. Inspired by [OpenWiki](https://github.com/langchain-ai/openwiki), it acts as an agent that explores your source tree, reads files, searches for patterns, and writes documentation covering architecture, modules, and APIs.

## Why WakaWiki?

Writing and maintaining documentation is tedious. WakaWiki delegates this to an LLM agent equipped with tools to navigate your codebase. The agent decides what to read, what to document, and where to write — all into a clean `wakawiki/` directory.

## How It Works

1. **Explore** — The agent lists files and directories to understand the project structure
2. **Read** — Reads key source files to understand the code
3. **Search** — Searches for patterns (functions, structs, traits) to find important code
4. **Write** — Outputs well-structured Markdown docs into `wakawiki/`
5. **Update** — On subsequent runs, only refreshes changed files

## Output Structure

```
wakawiki/
├── index.md            # Project overview
├── architecture.md     # High-level architecture
├── modules/            # Per-module documentation
│   ├── main.md
│   ├── agent.md
│   └── ...
└── .wakawiki.json      # Metadata for incremental updates
```

An `AGENTS.md` file is also created (or appended) with a reference block pointing coding agents to the `wakawiki/` directory.
