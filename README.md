[![Github Actions](https://github.com/FabienD/docker-stack/actions/workflows/dctl_cli.yml/badge.svg)](https://github.com/FabienD/docker-stack/actions)
[![GitHub release](https://img.shields.io/github/release/FabienD/docker-stack.svg)](https://github.com/FabienD/docker-stack/releases)
[![Codecov](https://codecov.io/github/FabienD/docker-stack/branch/main/graph/badge.svg?token=IH5NLYP8K4)](https://codecov.io/github/FabienD/docker-stack)
[![dependency status](https://deps.rs/repo/github/FabienD/docker-stack/status.svg?path=cli)](https://deps.rs/repo/github/FabienD/docker-stack?path=cli)
[![GitHub license](https://img.shields.io/github/license/FabienD/docker-stack.svg)](https://github.com/FabienD/docker-stack/blob/main/LICENSE)


# The docker Stack

This project is composed of a collection of usefull docker-compose files for web developpers. We also provide a standalone Cli tool to manage a list of docker compose files registered in a config.

[**Cli tool**](doc/dctl.md) and [**compose files collection**](doc/collection.md) are independants, you can use cli without the compose files and vice versa.

## Documentation

- Manage your local docker compose projects easilly with [the cli tool : dctl](doc/dctl.md)
- Your local stack for web development. [A collection of docker-compose files](doc/collection.md)

- The project [change log](CHANGELOG.md)

## "dctl" cli vs "docker compose" ?

With **dctl**, we relies on **a config file**, no need to have started or stopped containers of a docker-compose file to see and manage them, they are known. dctl use docker compose internaly, it's a wrapper around docker compose, so you can use all docker compose commands and arguments.


[![asciicast](https://asciinema.org/a/7VwsLtJmxzZ2HCkbim3kUQ21e.svg)](https://asciinema.org/a/7VwsLtJmxzZ2HCkbim3kUQ21e)

The cli can "manage" multiple docker-compose files (start, stop, down, restart, build and more) from **everywhere in your terminal**. With this tool, you can **avoid declaring multiple aliases** in your shell, and you can use the same command to start/stop/restart/ and so on for all your projects. The cli offers an **easy way to override default docker-compose** file for a project, you can also define **default arguments for all docker-compose commands** to avoid repeating them in the command line.

[See more](doc/dctl.md)

## Roadmap / next steps

- [x] ~~Build - Plublish the cli tool for multiples platforms (Windows, Mac, Linux), different architectures (x86, arm, arm64).~~
- [ ] Config - Add the possibility to automatically add env variables in the command line when execute docker compose.
- [ ] Default arguments - make it disablable by specifying a special argument.
- [ ] Default arguments - by project.
- [ ] Default arguments - clever merge default arguments with the ones specified in the command line.
- [ ] Cli - Add a "dctl" command to register/update a project using docker-compose.yml file.
- [ ] Cli - Add a "dctl" command to unregister a project using docker-compose.yaml file.
- [x] ~~Cli - improve check-config cli command, better presentation.~~
- [x] ~~Documentation - give cli examples, screenshots.~~
- [ ] Documentation - add examples of docker-compose files for local development.


## Contributing

Contributions are welcome, feel free to open an issue or a pull request.