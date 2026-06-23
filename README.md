<!-- SPDX-License-Identifier: Apache-2.0 -->

# Artesian for Zed

Memory control for AI agents inside [Zed](https://zed.dev). This extension registers
[Artesian](https://github.com/aquifer-labs/artesian)'s MCP server as a Zed **context server**, so
Zed's agent gains durable, qualify-gated memory over your own store: recall the high-signal slice,
store durable learnings, resume across compaction, and run loop-memory and orchestration kits.

## Requirements

The `artesian-mcp` binary must be on your `PATH`. Install via Homebrew (macOS/Linux):

```sh
brew install aquifer-labs/tap/artesian
# artesian-mcp is a symlink to the multi-call artesian binary
```

Or download a pre-built release binary from
[github.com/aquifer-labs/artesian/releases](https://github.com/aquifer-labs/artesian/releases)
and place it on your `PATH`.

### Configuration

`artesian-mcp` picks up its config from (in order of precedence):

1. `--config <path>` flag — pass an explicit `artesian.toml`
2. `ARTESIAN_MEMORY_ROOT` env var — override the `.artesian/` data directory
3. Default: creates `.artesian/` in the current working directory

For Qdrant or sqlite-vec backends, set the relevant env vars
(`QDRANT_URL`, `QDRANT_API_KEY`, etc.) before Zed launches. A wrapper script that
sources your shell rc is one way to ensure env vars reach the server:

```sh
# ~/artesian/run-artesian-mcp.sh
#!/usr/bin/env sh
set -eu
[ -f "$HOME/.zshrc" ] && . "$HOME/.zshrc"
exec artesian-mcp --config "$HOME/artesian.toml"
```

Then set `ARTESIAN_MCP_CMD` or point the extension at the script (see extension settings).

## Install

Until it lands in the Zed extension registry, install as a **dev extension**: open the command
palette → `zed: install dev extension` → select this folder.

## What you get

The context server exposes Artesian v0.4.x's full MCP tool surface to Zed's agent:

| Tool | Purpose |
|------|---------|
| `memory.find` | Hybrid retrieval — recall the high-signal slice for the current task |
| `memory.store` | Store a durable, reusable learning |
| `memory.context` | Compact index + targeted find hits in one call |
| `memory.answer` | Synthesize an answer from stored memory |
| `memory.anchor.get` / `memory.anchor.set` | Read/write named resume anchors |
| `session.checkpoint` | Write a resume packet before compaction or handoff |
| `session.resume` | Restore agent state from a prior checkpoint |
| `team.*` | Multi-agent coordination (spawn, route, aggregate) |
| `orchestrate.*` | Autonomous loop orchestration, including `orchestrate.loop` |
| `memory.dream` | Background OCF memory consolidation |

Bring your own backend (files / sqlite-vec / Qdrant). See
[Composability](https://github.com/aquifer-labs/artesian/blob/main/docs/composability.md) and
[Loop Engineering](https://github.com/aquifer-labs/artesian/blob/main/docs/loop-engineering.md).

## License

Apache-2.0.
