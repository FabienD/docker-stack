## 1.0.0-beta (2023-02-xx)

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
