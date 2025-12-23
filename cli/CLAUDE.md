# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Test Commands

```bash
# Build
cargo build              # Debug build
cargo build --release    # Release build (with LTO, optimized for size)

# Test
cargo test               # Run all tests
cargo test <test_name>   # Run specific test
cargo test -- --nocapture  # Show println! output

# Run locally
cargo run -- <args>      # Example: cargo run -- up myproject
```

## Architecture

The CLI wraps `docker compose` commands, adding centralized project management via a TOML config file.

### Module Structure

```
src/
├── main.rs              # Entry point, config loading
├── cli.rs               # Command routing (clap), dispatches to handlers
├── command.rs           # CommandHandler trait + module declarations
├── command/
│   ├── args.rs          # Declarative argument system (ArgDef, CommandDef)
│   ├── definitions.rs   # All 24 docker compose command definitions
│   ├── registry.rs      # Command registry with handlers
│   ├── cd.rs            # cd command (non-compose)
│   ├── completion.rs    # Shell completion
│   ├── config.rs        # Config check command (with YAML validation)
│   ├── infos.rs         # Project info command (parallel execution)
│   ├── register.rs      # Register new projects to config
│   └── unregister.rs    # Remove projects from config
├── parser/
│   └── config.rs        # TOML config parsing, DctlConfig, ComposeItem
└── utils/
    ├── docker.rs        # Container trait, Docker struct, command execution
    └── system.rs        # System::execute() - async process spawning
```

### Command Definition System (v2.0)

Commands are defined declaratively in `definitions.rs` using the argument system from `args.rs`:

```rust
// Example from definitions.rs
pub fn up_def() -> CommandDef {
    CommandDef {
        name: "up",
        about: "Create and start containers",
        needs_project: true,
        args: vec![
            ArgDef::Flag { id: "DETACH", long: "detach", short: Some('d'), help: "..." },
            ArgDef::Choice { id: "PULL", long: "pull", ..., choices: &["always", "missing", "never"] },
            ArgDef::Number { id: "TIMEOUT", long: "timeout", short: Some('t'), help: "..." },
            ArgDef::Services,
        ],
    }
}
```

#### Argument Types

| Type | Usage | Example |
|------|-------|---------|
| `ArgDef::Flag` | Boolean flags | `--detach`, `--build` |
| `ArgDef::Value` | String values | `--user root`, `--workdir /app` |
| `ArgDef::Choice` | Predefined choices (validated) | `--pull always\|missing\|never` |
| `ArgDef::Number` | Numeric values (validated >= 0) | `--timeout 30` |
| `ArgDef::Services` | Multiple service names | `web api db` |
| `ArgDef::Container` | Single container name | `web` |
| `ArgDef::ServiceWithCommand` | Service + command + args | `php bash -c "echo hi"` |

### Key Traits

- `CommandHandler` (`command.rs`): Interface for command handlers
  ```rust
  pub trait CommandHandler {
      fn name(&self) -> &'static str;
      fn cli(&self) -> Command;
      fn command_type(&self) -> CommandType;
      fn prepare(&self, args: &ArgMatches) -> Vec<OsString>;
  }
  ```
- `CliConfig` (`parser/config.rs`): Interface for config operations
- `Container` (`utils/docker.rs`): Interface for docker command execution

### Config Structure

Config file location: `~/.config/dctl/config.toml` (override with `DCTL_CONFIG_FILE_PATH` env var)

```toml
[main]
docker_bin = "/usr/bin/docker"
default_command_args = [
    { command_name = "up", command_args = ["-d", "--remove-orphans"] }
]

[[collections]]
alias = "myproject"
compose_files = ["/path/to/docker-compose.yml"]
enviroment_file = "/path/to/.env"  # optional
use_project_name = true            # optional, default true
description = "My project"         # optional
```

### Command Flow

1. `main.rs`: Load config from TOML, init `Docker` struct
2. `cli.rs:run()`: Parse command via clap, get project's `ComposeItem`
3. `registry.rs`: Find command handler by name
4. `Docker::compose()`: Build full command with config args + default args + CLI args
5. `System::execute()`: Spawn docker process asynchronously

### Adding a New Command

1. Add definition in `command/definitions.rs`:
   ```rust
   pub fn mycommand_def() -> CommandDef {
       CommandDef {
           name: "mycommand",
           about: "Description",
           needs_project: true,
           args: vec![/* ... */],
       }
   }
   ```

2. Add `CommandType` variant in `utils/docker.rs`:
   ```rust
   pub enum CommandType {
       // ...
       MyCommand,
   }
   ```

3. Register in `command/registry.rs`:
   ```rust
   define_command_from_def!(MyCommandCommand, MyCommand, mycommand_def);
   // Add to get_compose_commands()
   ```

4. Add tests in `command/definitions_tests.rs`

### Testing

Tests are colocated with modules (`#[cfg(test)] mod tests`). The `mockall` crate is used for mocking `System` in tests.

Key test files:
- `command/args.rs` - Argument system tests
- `command/definitions_tests.rs` - All 24 command definitions tests (65+ tests)
- `command/registry.rs` - Registry tests
- `command/register.rs` - Register command tests
- `command/unregister.rs` - Unregister command tests
- `parser/tests.rs` - Config parsing tests
- `utils/docker.rs` - Command preparation tests
- `utils/system_tests.rs` - System execution tests

Total: **148 unit tests**

### Supported Docker Compose Commands

All 24 docker compose commands are supported:
`build`, `config`, `create`, `down`, `events`, `exec`, `images`, `kill`, `logs`, `ls`, `pause`, `port`, `ps`, `pull`, `push`, `restart`, `rm`, `run`, `start`, `stop`, `top`, `unpause`, `up`, `watch`

### CLI-specific Commands

- `infos` - List all projects with status (parallel execution)
- `cd` - Print project directory path
- `check-config` - Validate config files (use `--validate` for YAML syntax check)
- `completion` - Generate shell completions
- `register` - Add project to config: `dctl register alias file1.yml [file2.yml...] [-e .env] [-d "desc"]`
- `unregister` - Remove project from config: `dctl unregister alias [--force]`