# Changelog

## 2.0.0 release (2025-12-23)

Major refactoring of the CLI architecture for better maintainability and compatibility with Docker Compose.

### Breaking Changes
* Internal API changes - command handlers now return `Vec<OsString>` instead of `Vec<&OsStr>`

### New Features
* **Declarative argument system**: New type-safe argument definitions with validation
  * `ArgDef::Flag` - Boolean flags (--flag)
  * `ArgDef::Value` - String values (--option value)
  * `ArgDef::Choice` - Predefined choices with validation (--pull always|missing|never)
  * `ArgDef::Number` - Numeric values with validation (--timeout 30)
  * `ArgDef::Services` - Multiple services support
  * `ArgDef::ServiceWithCommand` - Service + command + args (for exec/run)
* **Full exec/run command support**: Now correctly passes commands and arguments to containers
  * Example: `dctl exec myproject php bash -c "echo hello"`
  * Example: `dctl run --rm myproject php bin/console cache:clear`

### Improvements
* **65% code reduction**: Replaced 23 individual command files (~3500 lines) with declarative definitions (~1200 lines)
* **Docker Compose compatibility**: All 23 commands verified against official Docker Compose documentation
* **Better type validation**: Choice arguments reject invalid values, Number arguments reject negative/non-numeric values
* **Async execution**: Migrated to `tokio::process::Command` for better async support
* **133 unit tests**: Comprehensive test coverage for all commands

### Technical Changes
* New `CommandHandler` trait with registry pattern
* New `definitions.rs` with all command definitions
* New `args.rs` with declarative argument system
* Removed duplicate code across command files
* Fixed typos: `CommandOuput` → `CommandOutput`, `docker_commmand_arg` → `docker_command_arg`
* Error messages now output to stderr instead of stdout

## 1.5.2 release (2025-09-05)

* Update libraries (deps).
* Update version of docker image used in Collection :
  * Grafana 11.6.5
  * Loki 3.5.4
  * Promtail 3.5.4

## 1.5.1 release (2025-06-05)

* Update libraries (deps).
* Update version of docker image used in Collection :
  * Grafana 11.6.2
  * Loki 3.5.1
  * Promtail 3.5.1

## 1.5.0 release (2025-03-10)

* Add support for docker compose:
  * **port** command.
* Update libraries (deps).
* Update version of docker image used in Collection :
  * Grafana 11.5.2
  * Loki 3.4.2
  * Promtail 3.4.2

## 1.4.11 release (2025-02-08)

* Update libraries (deps).
* Update version of docker image used in Collection :
  * Traefik 3.3
  * Grafana 11.5.1
  * Loki 3.3.2
  * Promtail 3.3.2

## 1.4.10 release (2024-12-06)

* Update libraries (deps).
* Update version of docker image used in Collection :
  * Traefik 3.2
  * Grafana 11.4.0
  * Loki 3.3.1
  * Promtail 3.3.1

## 1.4.9 release (2024-09-29)

* Update libraries (deps).
* Update version of docker image used in Collection :
  * Traefik 3.1
  * Grafana 11.2.1
  * Loki 3.2.0
  * Promtail 3.2.0
  * Postgres 17
  * RabbitMq 4

## 1.4.8 release (2024-08-17)

* Update libraries (deps).
* Update version of docker image used in Collection :
  * Traefik 3.1
  * Grafana 11.1.4
  * Loki 3.0.1
  * Promtail 3.0.1

## 1.4.7 release (2024-06-06)

* Update libraries (deps).
* Update version of docker image use in Collection :
  * Grafana 11.0.0
  * Loki 3.0.0 -> need to update the configuration, and loki docker plugin at the same version.
  * Promtail 3.0.0

## 1.4.6 release (2024-04-05)

* Update libraries (deps).
* Update version of docker image used in Collection.

## 1.4.5 release (2024-03-06)

* Update libraries (deps). Security advisories

## 1.4.4 release (2024-02-17)

* Update libraries (deps).
* Update version of docker image used in Collection.

## 1.4.3 release (2023-12-23)

* Update libraries (deps).
* Update version of docker image used in Collection.

## 1.4.2 release (2023-12-01)

* Update libraries (deps).
* Update version of docker image used in Collection.
  
## 1.4.1 release (2023-11-03)

* Update libraries (deps).

## 1.4.0 release (2023-10-13)

* Add docker compose **watch** command support.
* Update libraries (deps).

## 1.3.5 release (2023-09-28)

* Update libraries (deps).

## 1.3.4 release (2023-09-01)

* Update libraries (deps).
  
## 1.3.3 release (2023-08-10)

* Update libraries (deps).

## 1.3.2 release (2023-07-08)

* Update libraries (deps).

## 1.3.1 release (2023-06-26)

* Fix dctl "infos" status for stopped containers.

## 1.3.0 release (2023-06-11)

* dctl "infos" command check config for each item.
* dctl "check-config" refactoring.
* Update libraries (deps).
  
## 1.2.0 release (2023-06-04)

* Add dctl "dry-run" command option.
* Update documentation.
* Update libraries (deps).

## 1.1.4 release (2023-05-27)

* Update libraries (deps).
* Update documentation.
  
## 1.1.3 release (2023-04-20)

* Update libraries (deps).

## 1.1.2 release (2023-03-19)

* New dclt CI workflow (Github Actions).
* Provide dctl for Windows, Linux and MacOS (6 binaries).
* Update libraries (deps).
  
## 1.1.1 release (2023-03-09)

* Fix default cli commands arguments.
* Update libraries (deps).

## 1.1.0 release (2023-02-25)

* Add command "check-config" to validate files in the configuration.
* Update libraries (deps).
* Update documentation.

## 1.0.1 release (2023-02-18)

* Update deps.

## 1.0.0 release (2023-02-14)

* Update documentation.

## 1.0.0-rc.1 release (2023-02-12)

* Default docker compose command arguments are configurable globally.
* Rewrite tests and documentation.
* Clean & refactor code (need to do more).

## 1.0.0-b2 (2023-02-04)

* dctl infos re-implemented, 3 status now available: running, stopped, partial-running.
* **TODO**: rewrite tests and documentation. Add configurable default command arguments.

## 1.0.0-b1 (2023-02-03)

* dctl implements all Docker compose command arguments.
* Add additional docker compose commands.
* Dependencies are updated.

## 0.6.1 (2022-11-24)

* Fix typo for Up command.

## 0.6.0 (2022-11-19)

* Add support for Run & Up commands. Now Start command do a the compoe start command, not the up one.

## 0.5.2 (2022-11-18)

* Update libraries

## 0.5.1 (2022-11-05)

* Update docker images to Rust 1.65 (Circle CI, and Build)
* Update libraries

## 0.5.0 (2022-10-16)

* Add cli shell completion
* Update documentation

## 0.4.0 (2022-10-14)

* Add running status in the dctl command list.
* Refactoring the code, add more tests.

## 0.3.0 (2022-10-12)

* dctl cli now supports "dctl logs" command.
* dctl can use alias (default) or directory name (for compose compatibilty) as project name. Cf. new config option "use_project_name".

## 0.2.1 (2022-10-02)

* Major upgrade of clap crate to v4.

## 0.2.0 (2022-10-02)

* Add additional features (Build, Exec, Ps & Cd comamnds).
* Little refactoring with more tests.

## 0.1.0 (2022-09-24)

* Initial release.

## Alpha 2 (2022-09-19)

* Environment file is not required anymore.
  
## Alpha 1 (2022-09-18)

* Initial Cli release. Start, Stop and Restart commands are available.
