[![Github Actions](https://github.com/FabienD/docker-stack/actions/workflows/dctl_cli.yml/badge.svg)](https://github.com/FabienD/docker-stack/actions)
[![GitHub release](https://img.shields.io/github/release/FabienD/docker-stack.svg)](https://github.com/FabienD/docker-stack/releases)
[![Codecov](https://codecov.io/github/FabienD/docker-stack/branch/main/graph/badge.svg?token=IH5NLYP8K4)](https://codecov.io/github/FabienD/docker-stack)
[![dependency status](https://deps.rs/repo/github/FabienD/docker-stack/status.svg?path=cli)](https://deps.rs/repo/github/FabienD/docker-stack?path=cli)
[![GitHub license](https://img.shields.io/github/license/FabienD/docker-stack.svg)](https://github.com/FabienD/docker-stack/blob/main/LICENSE)


# The docker Stack

This project is composed of a 2 things :

- A Docker Compose compatible CLI, but with more features; try it!
- A collection of Docker Compose files that aims to provide a functional common web stack for developers. It's easy to intergrate your own projects around the stack.

[**Cli tool**](doc/dctl.md) and [**compose files collection**](doc/collection.md) are independants, you can use cli without the compose files and vice versa.

## Documentation

- Manage your local docker compose projects easilly with [the cli tool : dctl](doc/dctl.md)
- Your local stack for web development. [A collection of docker-compose files](doc/collection.md)

- The project [change log](CHANGELOG.md)

## "dctl" cli vs "docker compose" ?

With **dctl**,

- no need to be in the project folder,
- no need to know new commands and arguments, "dclt" use the same ones as docker compose, you won't be lost,

With **dctl**, you can manage your project from everywhere in your terminal.

[![asciicast](https://asciinema.org/a/7VwsLtJmxzZ2HCkbim3kUQ21e.svg)](https://asciinema.org/a/7VwsLtJmxzZ2HCkbim3kUQ21e)

The cli can "manage" multiple docker-compose files (start, stop, down, restart, build and more) from **everywhere in your terminal**. With this tool, you can **avoid declaring multiple aliases** in your shell, and you can use the same command to start/stop/restart/ and so on for all your projects. The cli offers an **easy way to override default docker-compose** file for a project, you can also define **default arguments for all docker-compose commands** to avoid repeating them in the command line.

[See more](doc/dctl.md)

## Roadmap / next steps

- [ ] Documentation - add examples of docker-compose files for local development.

### v1

- [x] ~~Build - Plublish the cli tool for multiples platforms (Windows, Mac, Linux), different architectures (x86, arm, arm64).~~
- [x] ~~Cli - improve check-config cli command, better presentation.~~
- [x] ~~Documentation - give cli examples, screenshots.~~
- [ ] Cli - Add a "dctl" command to register/update a project using docker-compose.yml file.
- [ ] Cli - Add a "dctl" command to unregister a project using docker-compose.yaml file.
- [ ] Default arguments - make it disablable by specifying a special argument.
- [ ] Default arguments - by project.
- [ ] Default arguments - clever merge default arguments with the ones specified in the command line.

### v2

- [ ] Re-implement docker compose using Docker API instead of rely on "docker compose" plugin.


## Contributing

Contributions are welcome, feel free to open an issue or a pull request.