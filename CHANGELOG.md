# Changelog

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
