---
layout: home

hero:
  name: WakaWiki
  text: Agent-driven docs for your codebase
  tagline: Let LLMs explore, read, and write structured documentation automatically.
  actions:
    - theme: brand
      text: Get Started
      link: /guide/getting-started
    - theme: alt
      text: View on GitHub
      link: https://github.com/sonyarianto/wakawiki

  - html: |
      <p style="margin-top: 2rem; color: var(--vp-c-text-2); font-size: 0.875rem;">
        Inspired by <a href="https://github.com/langchain-ai/openwiki" target="_blank" rel="noopener" style="font-weight: 600;">OpenWiki</a>
      </p>

features:
  - icon: 🤖
    title: Agent-Driven
    details: LLM explores your codebase, reads files, searches for patterns, and writes structured docs into a wakawiki/ directory.
  - icon: 🔌
    title: Multiple Providers
    details: OpenAI, Anthropic, DeepSeek, OpenRouter, or local opencode (no API key needed). Bring your own LLM.
  - icon: 🔧
    title: Tool-Calling Agent Loop
    details: Built-in tools — list_files, read_file, search, write_doc — so the LLM can navigate and document any codebase.
  - icon: 🔄
    title: Incremental Updates
    details: Run wakawiki --update to refresh only what changed since the last run. Keeps docs in sync without redundant work.
  - icon: 📄
    title: AGENTS.md Integration
    details: Automatically creates or appends a reference block so coding agents know exactly where the docs live.
  - icon: ⚡
    title: CI Ready
    details: One-shot -p mode works in GitHub Actions. Schedule auto-updates via PR with zero manual effort.
---
