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


```toml
[main]
docker_bin = "/usr/bin/docker"

[[collections]]
alias = "stack_web"
use_project_name = true # Default value is true
description = "Docker stack - web components"
enviroment_file = "/home/fabien/workspace/infra/docker-stack/.env"
compose_files = [
    "/home/fabien/workspace/infra/docker-stack/web/docker-compose.yml",
]

[[collections]]
alias = "stack_logging"
description = "Docker stack - logging components"
enviroment_file = "/home/fabien/workspace/infra/docker-stack/.env"
compose_files = [
    "/home/fabien/workspace/infra/docker-stack/logging/docker-compose.yml",
]

[[collections]]
alias = "stack_tools"
description = "Docker stack - tools components"
enviroment_file = "/home/fabien/workspace/infra/docker-stack/.env"
compose_files = [
    "/home/fabien/workspace/infra/docker-stack/tools/docker-compose.yml",
]

[[collections]]
alias = "stack_data"
description = "Docker stack - data components"
enviroment_file = "/home/fabien/workspace/infra/docker-stack/.env"
compose_files = [
    "/home/fabien/workspace/infra/docker-stack/data/docker-compose.yml",
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

#### List registered docker-compose files

```bash
dctl list
```

#### Create and run containers by the project name

```bash
dctl up <name>
```

#### Start stopped containers by the project name

```bash
dctl start <name>
```

#### Stop containers by the project name

```bash
dctl stop <name>
```

#### Stop and remove all containers by the project name

```bash
dctl down <name>
```

#### Restart containers by the project name

```bash
dctl restart <name>
```

#### Show processus list by the project name

```bash
dctl ps <name>
```

#### Show containers logs by the project name

```bash
dctl logs <name> [service]
```

#### Build a container (or all buildable) by the project name

```bash
dctl build <name> [service]
```

#### Execute a subcommand in a container by the project name

```bash
dctl exec <name> <service> <subcommand>
```
#### Run a subcommand in a container by the project name

```bash
dctl exec <name> <service> <csubommand>
```


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
