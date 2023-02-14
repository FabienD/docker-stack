# Docker-stack

<<[Home](../README.md)>>

## "dctl" cli tools, a docker compose missing feature

The dctl sources can be found in the [cli](./cli/) path, it's write in Rust.

### The cli goals

The cli tool can "manage" multiple docker-compose files (start, stop, down, restart and build) from everywhere in your terminal.
All registered docker-compose files are stored in a configuration file (config.toml), by default in your home directory (~/.config/dctl/config.toml).

### Installation

You can download the binary from the [release page](https://github.com/FabienD/docker-stack/releases)

Or you can build it from source. You need to [install Rust and Cargo](https://www.rust-lang.org/tools/install) before.

```bash
cd cli && cargo build --release
```

### The config file

The config file is a [TOML](https://toml.io/en/) file, with the following structure.

Note that the **use_project_name**, **description** and the **environment file** are not mandatory.

**use_project_name** is true by default, docker compose will use the alias as project name. For a full compatibilty with project running without setting a [project name](https://github.com/compose-spec/compose-spec/blob/master/spec.md#name-top-level-element), set it to false, docker compose will use the directory name as project name.

Since 1.0.0, the **default_command_args** is a map of docker compose command and arguments. It's used as default arguments for all docker compose command.

```toml
[main]
docker_bin = "/usr/bin/docker"
default_command_args = [ 
    { command_name = "up", command_args = ["-d", "--remove-orphans"]} 
]

[[collections]]
alias = "stack_web"
use_project_name = true # Default value is true
description = "Docker stack - web components"
enviroment_file = "/home/fabien/workspace/infra/docker-stack/collection/.env"
compose_files = [
    "/home/fabien/workspace/infra/docker-stack/collection/web/docker-compose.yml",
]

[[collections]]
alias = "stack_logging"
description = "Docker stack - logging components"
enviroment_file = "/home/fabien/workspace/infra/docker-stackcollection/.env"
compose_files = [
    "/home/fabien/workspace/infra/docker-stack/collection/logging/docker-compose.yml",
]

[[collections]]
alias = "stack_tools"
description = "Docker stack - tools components"
enviroment_file = "/home/fabien/workspace/infra/docker-stack/collection/.env"
compose_files = [
    "/home/fabien/workspace/infra/docker-stack/collection/tools/docker-compose.yml",
]

[[collections]]
alias = "stack_data"
description = "Docker stack - data components"
enviroment_file = "/home/fabien/workspace/infra/docker-stack/collection/.env"
compose_files = [
    "/home/fabien/workspace/infra/docker-stack/collection/data/docker-compose.yml",
]

[[collections]]
alias = "project_name1"
enviroment_file = "/home/fabien/workspace/apps/project1/.env"
compose_files = [
    "/home/fabien/workspace/apps/project1/docker-compose.yml",
]

[[collections]]
alias = "project_name2"
use_project_name = false
description = "The project 2"
compose_files = [
    "/home/fabien/workspace/apps/project2/worker/docker-compose.yml",
    "/home/fabien/workspace/apps/project2/api/docker-compose.yml",
]
```

### dctl cli usage

```bash
dctl --help
```

Since version 1.0.0, dctl cli arguments in option are the same as docker compose arguments. 

Use the "help" command to see the available options by command.

Those following docker compose command are fully supported :

- build: Build all or selected service(s) for a project
- create: Creates containers for a service of the project
- down: Stop and remove containers, networks, images, and volumes for a project
- exec: Execute a command in a running service of the project
- events: Receive real time events from ontainers
- images: List images used by the created containers
- kill: Kill containers
- logs: View logs output from all containers or from selected services of the project
- ls: List running compose projects
- ps: List containers for a project or only selected service(s) of the project
- pause: Pause services
- pull: Pull service images
- push: Push services
- restart: Restart all containers for a project or only selected service(s) of the project
- rm: Removes stopped service containers
- run: Run a one-off command on a service
- start: Start all containers for a project or only selected service(s) of the project
- stop: Stop all containers for a project or only selected service(s) of the project
- top: Top on all containers for a project or only on selected service(s) of the project
- unpause: Unpause services
- up: Create and start containers for a project


#### List registered docker-compose files

```bash
dctl infos
```

For each registered project, the running status is displayed (running, stopped or partially running).
*Partially running* means that not all docker compose services of the project are running (It may be normal, depending of your compose file).

#### Show the path of a docker-compose by the project name

```bash
dctl cd <name>
```

This return the path name of the first docker-compose file of the collection.
We can't directly interact with the shell, you can use the command with the `cd` command.

```bash
cd "$(dctl cd <name>)"
```

Use a shell function to make it easier, for example in your .bashrc or .zshrc

```bash
function gocd() {
    cd "$(dctl cd $1)"
}
```

## Use dctl shell completion

Since version 0.5.0, dctl is able to generate a shell (Bash, Elvish, Fish, PowerShell, Zsh) completion script.

```bash
dctl completion <shell> > /path/to/completion/file
```

For example, to generate a completion script for **Bash**, you can run:

```bash
dctl completion bash > /etc/bash_completion.d/dctl
```

For **Zsh**, you can run:

```bash
dctl completion zsh > /usr/local/share/zsh/site-functions/_dctl
compinit
```

## Use the collection without the cli tool

You can use all of the docker-compose files without the cli, use docker command like this :

```bash
docker compose -f /docker-stack/web/docker-compose.yml --env-file /docker-stack/collection/.env up -d
```

Notice that tou use the docker compose V2 syntax.
