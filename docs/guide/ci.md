# GitHub Actions

Add this workflow to your repository to auto-update docs daily via PR:

```yaml
name: wakawiki update
on:
  schedule:
    - cron: '0 0 * * *'
  workflow_dispatch:

jobs:
  update:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1
      - run: cargo install --git https://github.com/sonyarianto/wakawiki.git
      - run: wakawiki --update
        env:
          WAKAWIKI_PROVIDER: ${{ secrets.WAKAWIKI_PROVIDER }}
          WAKAWIKI_API_KEY: ${{ secrets.WAKAWIKI_API_KEY }}
          WAKAWIKI_MODEL: ${{ secrets.WAKAWIKI_MODEL }}
      - uses: peter-evans/create-pull-request@v6
        with:
          title: 'docs: update wakawiki documentation'
          branch: wakawiki-update
```

## Required Secrets

Add these to your repository's **Settings > Secrets and variables > Actions**:

| Secret | Description |
|--------|-------------|
| `WAKAWIKI_PROVIDER` | e.g. `openai` |
| `WAKAWIKI_API_KEY` | Your API key |
| `WAKAWIKI_MODEL` | e.g. `gpt-4o` |

## Manual Trigger

The workflow includes `workflow_dispatch`, so you can also trigger it manually from the **Actions** tab.
