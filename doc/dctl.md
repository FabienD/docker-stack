# dctl CLI – The missing Docker Compose companion

<<[Home](../README.md)>>

## What is `dctl`?

`dctl` is a feature-rich CLI tool, written in Rust, that makes managing multiple Docker Compose projects effortless. It allows you to control all your local development stacks from anywhere in your terminal, with a consistent and simple interface.

- **Centralized management:** Register all your compose projects in a single config file.
- **No more aliases:** Use the same commands for all your projects, no matter where you are.
- **Override and customize:** Easily override compose files and set default arguments per command or project.

---

## Installation

- **Download the latest binary** from the [releases page](https://github.com/FabienD/docker-stack/releases)
- **Or build from source:**

```bash
cd cli && cargo build --release
```

Requires [Rust and Cargo](https://www.rust-lang.org/tools/install).

---

## Configuration

`dctl` uses a [TOML](https://toml.io/en/) config file, typically located at `~/.config/dctl/config.toml`.

**Example:**

```toml
[main]
docker_bin = "/usr/bin/docker"
default_command_args = [
    { command_name = "up", command_args = ["-d", "--remove-orphans"] }
]

[[collections]]
alias = "stack_web"
description = "Web stack components"
use_project_name = true # Optional, default: true
enviroment_file = "/path/to/.env"
compose_files = [
    "/path/to/web/docker-compose.yml"
]

# ... more collections ...
```

- **alias:** Unique name for your project.
- **use_project_name:** (Optional) If true, uses the alias as the Docker Compose project name.
- **description:** (Optional) Free text description.
- **enviroment_file:** (Optional) Path to your .env file.
- **compose_files:** List of compose files for the project.
- **default_command_args:** (Optional) Default arguments per Docker Compose command.

---

## Usage

### Check your configuration

```bash
dctl check-config
```

Validates your config file and all referenced compose files.

### List registered projects

```bash
dctl infos
```

Shows all registered projects and their running status (running, stopped, or partially running).

### Show the path to a project's compose file

```bash
dctl cd <alias>
```

Returns the path to the first compose file for the given project. Combine with `cd` in your shell:

```bash
cd "$(dctl cd <alias>)"
```

Add a helper function to your `.zshrc` or `.bashrc`:

```bash
function gocd() {
    cd "$(dctl cd $1)"
}
```

### Shell completion

Generate completion scripts for Bash, Zsh, Fish, etc.:

```bash
dctl completion <shell> > /path/to/completion/file
```

Example for Zsh:

```bash
dctl completion zsh > /usr/local/share/zsh/site-functions/_dctl
compinit
```

---

## Supported Docker Compose commands

`dctl` supports all major Docker Compose commands, including:

- up, down, start, stop, restart, build, exec, logs, ps, images, pull, push, rm, run, ls, pause, unpause, kill, create, events, top

Arguments and options are passed through to Docker Compose, so you can use `dctl` just like the original tool.

## Contributing

Contributions are welcome! Open an issue or a pull request to help improve `dctl`.