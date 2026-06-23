// SPDX-License-Identifier: Apache-2.0
use zed_extension_api::{self as zed, Command, ContextServerId, Result};

/// Artesian extension for Zed.
///
/// Registers Artesian as an MCP context server, giving Zed's AI agent durable, qualify-gated
/// memory over your own store: recall the high-signal slice (`memory.find`), store durable
/// learnings (`memory.store`), resume across compaction (`memory.anchor.*`), synthesize answers
/// (`memory.answer`), checkpoint and resume sessions (`session.checkpoint` / `session.resume`),
/// and the full loop-memory and orchestration kit.
///
/// Requires the `artesian-mcp` binary in PATH — install via Homebrew:
///   `brew install aquifer-labs/tap/artesian`
/// or download a pre-built binary from https://github.com/aquifer-labs/artesian/releases .
/// Configure via `--config artesian.toml`, `ARTESIAN_MEMORY_ROOT`, or backend env vars.
struct ArtesianExtension;

impl zed::Extension for ArtesianExtension {
    fn new() -> Self {
        ArtesianExtension
    }

    fn context_server_command(
        &mut self,
        _id: &ContextServerId,
        _project: &zed::Project,
    ) -> Result<Command> {
        Ok(Command {
            command: "artesian-mcp".to_string(),
            args: Vec::new(),
            env: Vec::new(),
        })
    }
}

zed::register_extension!(ArtesianExtension);
