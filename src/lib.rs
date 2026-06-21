// SPDX-License-Identifier: Apache-2.0
use zed_extension_api::{self as zed, Command, ContextServerId, Result};

/// Artesian extension for Zed.
///
/// Registers Artesian as an MCP context server, giving Zed's AI agent durable, qualify-gated
/// memory over your own store: recall the high-signal slice (`memory.find`), commit durable
/// learnings (`memory.commit`), resume across compaction (`memory.anchor.*`), and the
/// loop-memory kit.
///
/// Requires the `artesian-mcp` binary in PATH
/// (`cargo install --git https://github.com/aquifer-labs/artesian artesian-mcp`, or build from
/// source). Configure via `ARTESIAN_HOME` or an `artesian.toml`.
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
