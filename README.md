[![Github Actions](https://github.com/FabienD/docker-stack/actions/workflows/dctl_cli.yml/badge.svg)](https://github.com/FabienD/docker-stack/actions)
[![GitHub release](https://img.shields.io/github/release/FabienD/docker-stack.svg)](https://github.com/FabienD/docker-stack/releases)
[![Codecov](https://codecov.io/github/FabienD/docker-stack/branch/main/graph/badge.svg?token=IH5NLYP8K4)](https://codecov.io/github/FabienD/docker-stack)
[![dependency status](https://deps.rs/repo/github/FabienD/docker-stack/status.svg?path=cli)](https://deps.rs/repo/github/FabienD/docker-stack?path=cli)
[![GitHub license](https://img.shields.io/github/license/FabienD/docker-stack.svg)](https://github.com/FabienD/docker-stack/blob/main/LICENSE)

# Docker Stack

**Docker Stack** is a developer-oriented project that provides:

- **A powerful CLI tool (`dctl`)**: A drop-in replacement for Docker Compose, with extra features and improved usability.
- **A curated collection of Docker Compose files**: Ready-to-use configurations for a complete local development stack.

You can use the CLI and the compose files independently or together, depending on your needs.

---

## Overview

- **[CLI Tool Documentation](doc/dctl.md):** Manage your local Docker Compose projects more easily with `dctl`.
- **[Compose Files Collection](doc/collection.md):** Quickly set up a local web development stack with pre-configured services.
- **[Changelog](CHANGELOG.md):** See what's new.

---

## Why use `dctl` instead of `docker compose`?

- No need to be in your project folder.
- Uses the same commands and arguments as Docker Compose—no new learning curve.
- Manage multiple projects from anywhere in your terminal.
- Easily override default compose files and set default arguments for all commands.

[![asciicast](https://asciinema.org/a/7VwsLtJmxzZ2HCkbim3kUQ21e.svg)](https://asciinema.org/a/7VwsLtJmxzZ2HCkbim3kUQ21e)

With `dctl`, you can start, stop, restart, build, and manage multiple Docker Compose projects from anywhere, without the need for custom shell aliases. The CLI also allows you to define default arguments and override compose files per project.

---

## Roadmap

- [ ] Add more documentation and examples for local development.
- [ ] Add commands to register/unregister projects using docker-compose files.
- [ ] Make default arguments optional and project-specific.
- [ ] Improve merging of default and command-line arguments.

### v1

- [x] Publish the CLI tool for multiple platforms (Windows, Mac, Linux) and architectures (x86, arm, arm64).
- [x] Improve the `check-config` command.
- [x] Add CLI examples and screenshots.

### v2

- [ ] Refactor the CLI tool for better architecture and code quality.

---

## Contributing

Contributions are welcome! Feel free to open an issue or a pull request.

---

## License

This project is licensed under the [MIT License](LICENSE).