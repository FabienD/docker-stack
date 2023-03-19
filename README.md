[![Github Actions](https://github.com/FabienD/docker-stack/actions/workflows/dctl_cli.yml/badge.svg)](https://github.com/FabienD/docker-stack/actions)
[![Codecov](https://codecov.io/github/FabienD/docker-stack/branch/main/graph/badge.svg?token=IH5NLYP8K4)](https://codecov.io/github/FabienD/docker-stack)
[![GitHub release](https://img.shields.io/github/release/FabienD/docker-stack.svg)](https://github.com/FabienD/docker-stack/releases)
[![GitHub license](https://img.shields.io/github/license/FabienD/docker-stack.svg)](https://github.com/FabienD/docker-stack/blob/main/LICENSE)


# The docker Stack

This project is composed of a collection of usefull docker-compose files for web developpers. We also provide a standalone Cli tool to manage a list of docker compose files registered in a config.

[**Cli tool**](doc/dctl.md) and [**compose files collection**](doc/collection.md) are independants, you can use cli without the compose files and vice versa.

## Documentation

- Manage your local docker compose projects easilly with [the cli tool : dctl](doc/dctl.md)
- Your local stack for web development. [A collection of docker-compose files](doc/collection.md)

- The project [change log](CHANGELOG.md)

## "dctl" cli vs "docker compose" ?

With **dctl**, we relies on **a config file**, no need to have started or stopped containers of a docker-compose file to see and manage them, they are known. dctl use docker compose internaly.

Docker compose command need to have started or stopped containers to know and manage them with their project name.

## Roadmap / next steps

- [x] Build - Plublish the cli tool for multiples platforms (Windows, Mac, Linux), different architectures (x86, arm, arm64).
- [ ] Config - Add the possibility to automatically add env variables in the command line when execute docker compose.
- [ ] Default arguments - make it disablable by specifying a special argument.
- [ ] Default arguments - merge default arguments with the ones specified in the command line.
- [ ] Cli - Add a "dctl" command to register/update a project using docker-compose.yml file.
- [ ] Cli - Add a "dctl" command to unregister a project using docker-compose.yaml file.
- [ ] Cli - improve check-config cli command, better presentation.
- [ ] Documentation - give cli examples, screenshots.
- [ ] Documentation - add examples of docker-compose files for local development.
