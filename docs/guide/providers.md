# Providers

WakaWiki supports multiple LLM providers. Choose one during `wakawiki --init`.

## Supported Providers

| Provider | API Key Required | Notes |
|----------|:-----------------:|-------|
| OpenAI | Yes | GPT-4, GPT-4o, o-series models |
| Anthropic | Yes | Claude Sonnet, Claude Opus |
| DeepSeek | Yes | OpenAI-compatible API |
| OpenRouter | Yes | Unified API gateway for many models |
| opencode | **No** | Uses your local `opencode` CLI |
| Custom | Yes | Any OpenAI-compatible endpoint |

## opencode Provider

If you already use [opencode](https://github.com/anomalyco/opencode), you can run WakaWiki with zero API keys. WakaWiki shells out to your local `opencode` CLI and uses whatever configuration you already have set up there.

```bash
wakawiki --init    # select "opencode"
wakawiki           # uses your existing opencode setup
```

## Custom Provider

For any OpenAI-compatible API (Ollama, vLLM, LM Studio, etc.), select "custom" during `wakawiki --init` and provide your base URL.

```bash
# Example: using a local Ollama instance
WAKAWIKI_PROVIDER=custom
WAKAWIKI_API_KEY=ollama
WAKAWIKI_BASE_URL=http://localhost:11434/v1
WAKAWIKI_MODEL=llama3
```
