<!-- SPDX-License-Identifier: Apache-2.0 -->

# Artesian for Zed

Memory control for AI agents inside [Zed](https://zed.dev). This extension registers
[Artesian](https://github.com/aquifer-labs/artesian)'s MCP server as a Zed **context server**, so
Zed's agent gains durable, qualify-gated memory over your own store: recall the high-signal slice,
commit durable learnings, resume across compaction, and run loop-memory kits.

## Requirements

The `artesian-mcp` binary must be on your `PATH`:

```sh
cargo install --git https://github.com/aquifer-labs/artesian artesian-mcp
# or build from source and put target/release/artesian-mcp on your PATH
```

Configure with `ARTESIAN_HOME` (data/config home) or an `artesian.toml`. See the
[Artesian docs](https://github.com/aquifer-labs/artesian/tree/main/docs).

## Install

Until it lands in the Zed extension registry, install as a **dev extension**: open the command
palette → `zed: install dev extension` → select this folder.

## What you get

The context server exposes Artesian's memory tools to Zed's agent — `memory.find`,
`memory.commit`, `memory.anchor.*`, and the loop-memory kit. Bring your own backend
(files / sqlite-vec / Qdrant). See
[Composability](https://github.com/aquifer-labs/artesian/blob/main/docs/composability.md) and
[Loop Engineering](https://github.com/aquifer-labs/artesian/blob/main/docs/loop-engineering.md).

## License

Apache-2.0.
