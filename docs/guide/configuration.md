# Configuration

Configuration is stored in `~/.wakawiki/.env`. You can also override any value with environment variables.

## Config File

```ini
WAKAWIKI_PROVIDER=openai
WAKAWIKI_API_KEY=sk-...
WAKAWIKI_MODEL=gpt-4o
WAKAWIKI_BASE_URL=https://api.openai.com/v1
```

## Environment Variables

| Variable | Description | Required |
|----------|-------------|:--------:|
| `WAKAWIKI_PROVIDER` | LLM provider (`openai`, `anthropic`, `deepseek`, `openrouter`, `opencode`, `custom`) | Yes |
| `WAKAWIKI_API_KEY` | API key for the provider (not needed for `opencode`) | Depends on provider |
| `WAKAWIKI_MODEL` | Model name to use (e.g. `gpt-4o`, `claude-sonnet-4-20250514`) | No |
| `WAKAWIKI_BASE_URL` | Custom base URL (for `custom` provider or proxies) | No |

## Reconfigure

Run `wakawiki --init` again at any time to change your provider or settings. The existing config will be overwritten.
