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

The context server exposes Artesian v0.5.x's full MCP tool surface to Zed's agent:

| Tool | Purpose |
|------|---------|
| `memory.find` | Hybrid retrieval — recall the high-signal slice for the current task |
| `memory.store` | Store a durable, reusable learning |
| `memory.context` | Compact index + targeted find hits in one call |
| `memory.answer` | Synthesize an answer from stored memory |
| `memory.anchor.get` / `memory.anchor.set` | Read/write named resume anchors |
| `memory.session.checkpoint` | Write a resume packet before compaction or handoff |
| `memory.session.resume` | Restore agent state from a prior checkpoint |
| `memory.session.resume_by_task` | Resume session anchored to a specific task ID |
| `memory.savings` | Retrieve cost/token savings report for this session |
| `memory.learn` | Explicitly promote an observation to long-term memory |
| `memory.skills` | List or look up stored procedural skills |
| `team.create` | Create a named multi-agent team |
| `team.spawn` | Spawn a worker agent into a team |
| `team.task.create` / `team.task.assign` / `team.task.status` | Create, assign, and track team tasks |
| `team.message` | Send a message to a team or member |
| `team.status` | Get overall team status |
| `team.cleanup` / `team.gc` | Tear down a team or garbage-collect stale state |
| `team.presence` | Report or query agent presence |
| `team.lane.*` | Manage named work lanes within a team |
| `orchestrate.bind` | Bind an orchestrator to a loop context |
| `orchestrate.delegate` | Delegate a subtask to another agent |
| `orchestrate.handoff` | Perform a structured handoff to another agent |
| `orchestrate.status` | Query orchestrator state |
| `orchestrate.loop` | Autonomous loop orchestration |
| `memory.dream` | Background OCF memory consolidation |

Bring your own backend (files / sqlite-vec / Qdrant). See
[Composability](https://github.com/aquifer-labs/artesian/blob/main/docs/composability.md) and
[Loop Engineering](https://github.com/aquifer-labs/artesian/blob/main/docs/loop-engineering.md).

## Troubleshooting

### Tools didn't refresh after upgrading Artesian

When you run `brew upgrade artesian`, the brew formula updates the binary on disk. However, Zed's
Claude agent may still have a **running `artesian-mcp` process from the old binary** — a
long-running MCP server keeps its file handle open on the old executable until restarted.

**Symptoms:** `claude mcp list` shows `✓ Connected` but `memory.*` tools behave as before the
upgrade (e.g., new tools are missing, or old bugs persist).

**Why `claude mcp list` is misleading:** The health-check reflects a fresh connection handshake,
not the agent's live in-session tool registration. The agent loads tool definitions when the
session starts — a stale process keeps the old tool list.

**Fix:**

1. Close the current Claude chat in Zed and open a new one (this spawns a fresh `artesian-mcp`
   from the updated binary).
2. Or manually kill stale processes: `pkill -f artesian-mcp` then restart the agent.
3. Future: `artesian update --restart-stale` will automate this (not yet released).

**Note:** Because Artesian uses a single multi-call binary (`artesian-mcp` is the same binary as
`artesian` invoked as the MCP subcommand), `brew upgrade` always updates the MCP — you just need
to restart any live sessions.

## License

Apache-2.0.
