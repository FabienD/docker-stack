//! Docker Compose command definitions
//!
//! This module contains all docker compose command definitions using the declarative
//! argument system. Each definition is compatible with the official Docker Compose CLI.
//!
//! Reference: https://docs.docker.com/reference/cli/docker/compose/

use super::args::{ArgDef, CommandDef};

// ============================================================================
// docker compose build
// https://docs.docker.com/reference/cli/docker/compose/build/
// ============================================================================
pub fn build_def() -> CommandDef {
    CommandDef {
        name: "build",
        about: "Build or rebuild services",
        needs_project: true,
        args: vec![
            ArgDef::Value {
                id: "BUILD_ARG",
                long: "build-arg",
                short: None,
                help: "Set build-time variables for services",
            },
            ArgDef::Value {
                id: "BUILDER",
                long: "builder",
                short: None,
                help: "Set builder to use",
            },
            ArgDef::Flag {
                id: "CHECK",
                long: "check",
                short: None,
                help: "Check build configuration",
            },
            ArgDef::Value {
                id: "MEMORY",
                long: "memory",
                short: Some('m'),
                help: "Set memory limit for the build container (not supported by BuildKit)",
            },
            ArgDef::Flag {
                id: "NO_CACHE",
                long: "no-cache",
                short: None,
                help: "Do not use cache when building the image",
            },
            ArgDef::Flag {
                id: "PRINT",
                long: "print",
                short: None,
                help: "Print equivalent bake file",
            },
            ArgDef::Choice {
                id: "PROGRESS",
                long: "progress",
                short: None,
                help: "Set type of progress output",
                choices: &["auto", "tty", "plain", "quiet"],
            },
            ArgDef::Flag {
                id: "PROVENANCE",
                long: "provenance",
                short: None,
                help: "Add a provenance attestation",
            },
            ArgDef::Flag {
                id: "PULL",
                long: "pull",
                short: None,
                help: "Always attempt to pull a newer version of the image",
            },
            ArgDef::Flag {
                id: "PUSH",
                long: "push",
                short: None,
                help: "Push service images",
            },
            ArgDef::Flag {
                id: "QUIET",
                long: "quiet",
                short: Some('q'),
                help: "Don't print anything to STDOUT",
            },
            ArgDef::Flag {
                id: "SBOM",
                long: "sbom",
                short: None,
                help: "Add a SBOM attestation",
            },
            ArgDef::Value {
                id: "SSH",
                long: "ssh",
                short: None,
                help: "Set SSH authentications used when building service images",
            },
            ArgDef::Flag {
                id: "WITH_DEPS",
                long: "with-dependencies",
                short: None,
                help: "Also build dependencies (transitively)",
            },
            ArgDef::Services,
        ],
    }
}

// ============================================================================
// docker compose create
// https://docs.docker.com/reference/cli/docker/compose/create/
// ============================================================================
pub fn create_def() -> CommandDef {
    CommandDef {
        name: "create",
        about: "Creates containers for a service",
        needs_project: true,
        args: vec![
            ArgDef::Flag {
                id: "BUILD",
                long: "build",
                short: None,
                help: "Build images before starting containers",
            },
            ArgDef::Flag {
                id: "FORCE_RECREATE",
                long: "force-recreate",
                short: None,
                help: "Recreate containers even if their configuration and image haven't changed",
            },
            ArgDef::Flag {
                id: "NO_BUILD",
                long: "no-build",
                short: None,
                help: "Don't build an image, even if it's policy",
            },
            ArgDef::Flag {
                id: "NO_RECREATE",
                long: "no-recreate",
                short: None,
                help: "If containers already exist, don't recreate them",
            },
            ArgDef::Choice {
                id: "PULL",
                long: "pull",
                short: None,
                help: "Pull image before running",
                choices: &["always", "missing", "never", "build"],
            },
            ArgDef::Flag {
                id: "QUIET_PULL",
                long: "quiet-pull",
                short: None,
                help: "Pull without printing progress information",
            },
            ArgDef::Flag {
                id: "REMOVE_ORPHANS",
                long: "remove-orphans",
                short: None,
                help: "Remove containers for services not defined in the Compose file",
            },
            ArgDef::Value {
                id: "SCALE",
                long: "scale",
                short: None,
                help: "Scale SERVICE to NUM instances",
            },
            ArgDef::Flag {
                id: "YES",
                long: "yes",
                short: Some('y'),
                help: "Assume 'yes' as answer to all prompts",
            },
            ArgDef::Services,
        ],
    }
}

// ============================================================================
// docker compose down
// https://docs.docker.com/reference/cli/docker/compose/down/
// ============================================================================
pub fn down_def() -> CommandDef {
    CommandDef {
        name: "down",
        about: "Stop and remove containers, networks",
        needs_project: true,
        args: vec![
            ArgDef::Flag {
                id: "REMOVE_ORPHANS",
                long: "remove-orphans",
                short: None,
                help: "Remove containers for services not defined in the Compose file",
            },
            ArgDef::Choice {
                id: "RMI",
                long: "rmi",
                short: None,
                help: "Remove images used by services",
                choices: &["local", "all"],
            },
            ArgDef::Number {
                id: "TIMEOUT",
                long: "timeout",
                short: Some('t'),
                help: "Specify a shutdown timeout in seconds",
            },
            ArgDef::Flag {
                id: "VOLUMES",
                long: "volumes",
                short: Some('v'),
                help: "Remove named volumes declared in the volumes section",
            },
            ArgDef::Services,
        ],
    }
}

// ============================================================================
// docker compose events
// https://docs.docker.com/reference/cli/docker/compose/events/
// ============================================================================
pub fn events_def() -> CommandDef {
    CommandDef {
        name: "events",
        about: "Receive real time events from containers",
        needs_project: true,
        args: vec![
            ArgDef::Flag {
                id: "JSON",
                long: "json",
                short: None,
                help: "Output events as a stream of json objects",
            },
            ArgDef::Services,
        ],
    }
}

// ============================================================================
// docker compose exec
// https://docs.docker.com/reference/cli/docker/compose/exec/
// ============================================================================
pub fn exec_def() -> CommandDef {
    CommandDef {
        name: "exec",
        about: "Execute a command in a running container",
        needs_project: true,
        args: vec![
            ArgDef::Flag {
                id: "DETACH",
                long: "detach",
                short: Some('d'),
                help: "Run command in the background",
            },
            ArgDef::Value {
                id: "ENV",
                long: "env",
                short: Some('e'),
                help: "Set environment variables",
            },
            ArgDef::Number {
                id: "INDEX",
                long: "index",
                short: None,
                help: "Index of the container if service has multiple replicas",
            },
            ArgDef::Flag {
                id: "NO_TTY",
                long: "no-TTY",
                short: Some('T'),
                help: "Disable pseudo-TTY allocation",
            },
            ArgDef::Flag {
                id: "PRIVILEGED",
                long: "privileged",
                short: None,
                help: "Give extended privileges to the process",
            },
            ArgDef::Value {
                id: "USER",
                long: "user",
                short: Some('u'),
                help: "Run the command as this user",
            },
            ArgDef::Value {
                id: "WORKDIR",
                long: "workdir",
                short: Some('w'),
                help: "Path to workdir directory for this command",
            },
            ArgDef::ServiceWithCommand,
        ],
    }
}

// ============================================================================
// docker compose images
// https://docs.docker.com/reference/cli/docker/compose/images/
// ============================================================================
pub fn images_def() -> CommandDef {
    CommandDef {
        name: "images",
        about: "List images used by the created containers",
        needs_project: true,
        args: vec![
            ArgDef::Choice {
                id: "FORMAT",
                long: "format",
                short: None,
                help: "Format the output",
                choices: &["table", "json"],
            },
            ArgDef::Flag {
                id: "QUIET",
                long: "quiet",
                short: Some('q'),
                help: "Only display IDs",
            },
            ArgDef::Services,
        ],
    }
}

// ============================================================================
// docker compose kill
// https://docs.docker.com/reference/cli/docker/compose/kill/
// ============================================================================
pub fn kill_def() -> CommandDef {
    CommandDef {
        name: "kill",
        about: "Force stop service containers",
        needs_project: true,
        args: vec![
            ArgDef::Flag {
                id: "REMOVE_ORPHANS",
                long: "remove-orphans",
                short: None,
                help: "Remove containers for services not defined in the Compose file",
            },
            ArgDef::Value {
                id: "SIGNAL",
                long: "signal",
                short: Some('s'),
                help: "SIGNAL to send to the container (default: SIGKILL)",
            },
            ArgDef::Services,
        ],
    }
}

// ============================================================================
// docker compose logs
// https://docs.docker.com/reference/cli/docker/compose/logs/
// ============================================================================
pub fn logs_def() -> CommandDef {
    CommandDef {
        name: "logs",
        about: "View output from containers",
        needs_project: true,
        args: vec![
            ArgDef::Flag {
                id: "FOLLOW",
                long: "follow",
                short: Some('f'),
                help: "Follow log output",
            },
            ArgDef::Number {
                id: "INDEX",
                long: "index",
                short: None,
                help: "Index of the container if service has multiple replicas",
            },
            ArgDef::Flag {
                id: "NO_COLOR",
                long: "no-color",
                short: None,
                help: "Produce monochrome output",
            },
            ArgDef::Flag {
                id: "NO_LOG_PREFIX",
                long: "no-log-prefix",
                short: None,
                help: "Don't print prefix in logs",
            },
            ArgDef::Value {
                id: "SINCE",
                long: "since",
                short: None,
                help: "Show logs since timestamp (e.g. 2013-01-02T13:23:37Z) or relative (e.g. 42m)",
            },
            ArgDef::Value {
                id: "TAIL",
                long: "tail",
                short: Some('n'),
                help: "Number of lines to show from the end of the logs",
            },
            ArgDef::Flag {
                id: "TIMESTAMPS",
                long: "timestamps",
                short: Some('t'),
                help: "Show timestamps",
            },
            ArgDef::Value {
                id: "UNTIL",
                long: "until",
                short: None,
                help: "Show logs before a timestamp or relative time",
            },
            ArgDef::Services,
        ],
    }
}

// ============================================================================
// docker compose ls
// https://docs.docker.com/reference/cli/docker/compose/ls/
// ============================================================================
pub fn ls_def() -> CommandDef {
    CommandDef {
        name: "ls",
        about: "List running compose projects",
        needs_project: false, // ls doesn't need a project
        args: vec![
            ArgDef::Flag {
                id: "ALL",
                long: "all",
                short: Some('a'),
                help: "Show all stopped Compose projects",
            },
            ArgDef::Value {
                id: "FILTER",
                long: "filter",
                short: None,
                help: "Filter output based on conditions provided",
            },
            ArgDef::Choice {
                id: "FORMAT",
                long: "format",
                short: None,
                help: "Format the output",
                choices: &["table", "json"],
            },
            ArgDef::Flag {
                id: "QUIET",
                long: "quiet",
                short: Some('q'),
                help: "Only display IDs",
            },
        ],
    }
}

// ============================================================================
// docker compose pause
// https://docs.docker.com/reference/cli/docker/compose/pause/
// ============================================================================
pub fn pause_def() -> CommandDef {
    CommandDef {
        name: "pause",
        about: "Pause services",
        needs_project: true,
        args: vec![ArgDef::Services],
    }
}

// ============================================================================
// docker compose port
// https://docs.docker.com/reference/cli/docker/compose/port/
// ============================================================================
pub fn port_def() -> CommandDef {
    CommandDef {
        name: "port",
        about: "Print the public port for a port binding",
        needs_project: true,
        args: vec![
            ArgDef::Number {
                id: "INDEX",
                long: "index",
                short: None,
                help: "Index of the container if service has multiple replicas",
            },
            ArgDef::Choice {
                id: "PROTOCOL",
                long: "protocol",
                short: None,
                help: "Protocol to use",
                choices: &["tcp", "udp"],
            },
            ArgDef::Container,
        ],
    }
}

// ============================================================================
// docker compose ps
// https://docs.docker.com/reference/cli/docker/compose/ps/
// ============================================================================
pub fn ps_def() -> CommandDef {
    CommandDef {
        name: "ps",
        about: "List containers",
        needs_project: true,
        args: vec![
            ArgDef::Flag {
                id: "ALL",
                long: "all",
                short: Some('a'),
                help: "Show all stopped containers",
            },
            ArgDef::Value {
                id: "FILTER",
                long: "filter",
                short: None,
                help: "Filter services by a property",
            },
            ArgDef::Choice {
                id: "FORMAT",
                long: "format",
                short: None,
                help: "Format the output",
                choices: &["table", "json"],
            },
            ArgDef::Flag {
                id: "NO_TRUNC",
                long: "no-trunc",
                short: None,
                help: "Don't truncate output",
            },
            ArgDef::Flag {
                id: "ORPHANS",
                long: "orphans",
                short: None,
                help: "Include orphaned services",
            },
            ArgDef::Flag {
                id: "QUIET",
                long: "quiet",
                short: Some('q'),
                help: "Only display IDs",
            },
            ArgDef::Flag {
                id: "SERVICES",
                long: "services",
                short: None,
                help: "Display services",
            },
            ArgDef::Choice {
                id: "STATUS",
                long: "status",
                short: None,
                help: "Filter services by status",
                choices: &["paused", "restarting", "removing", "running", "dead", "created", "exited"],
            },
            ArgDef::Services,
        ],
    }
}

// ============================================================================
// docker compose pull
// https://docs.docker.com/reference/cli/docker/compose/pull/
// ============================================================================
pub fn pull_def() -> CommandDef {
    CommandDef {
        name: "pull",
        about: "Pull service images",
        needs_project: true,
        args: vec![
            ArgDef::Flag {
                id: "IGNORE_BUILDABLE",
                long: "ignore-buildable",
                short: None,
                help: "Ignore images that can be built",
            },
            ArgDef::Flag {
                id: "IGNORE_PULL_FAILURES",
                long: "ignore-pull-failures",
                short: None,
                help: "Pull what it can and ignores images with pull failures",
            },
            ArgDef::Flag {
                id: "INCLUDE_DEPS",
                long: "include-deps",
                short: None,
                help: "Also pull services declared as dependencies",
            },
            ArgDef::Choice {
                id: "POLICY",
                long: "policy",
                short: None,
                help: "Apply pull policy",
                choices: &["missing", "always"],
            },
            ArgDef::Flag {
                id: "QUIET",
                long: "quiet",
                short: Some('q'),
                help: "Pull without printing progress information",
            },
            ArgDef::Services,
        ],
    }
}

// ============================================================================
// docker compose push
// https://docs.docker.com/reference/cli/docker/compose/push/
// ============================================================================
pub fn push_def() -> CommandDef {
    CommandDef {
        name: "push",
        about: "Push service images",
        needs_project: true,
        args: vec![
            ArgDef::Flag {
                id: "IGNORE_PUSH_FAILURES",
                long: "ignore-push-failures",
                short: None,
                help: "Push what it can and ignores images with push failures",
            },
            ArgDef::Flag {
                id: "INCLUDE_DEPS",
                long: "include-deps",
                short: None,
                help: "Also push images of services declared as dependencies",
            },
            ArgDef::Flag {
                id: "QUIET",
                long: "quiet",
                short: Some('q'),
                help: "Push without printing progress information",
            },
            ArgDef::Services,
        ],
    }
}

// ============================================================================
// docker compose restart
// https://docs.docker.com/reference/cli/docker/compose/restart/
// ============================================================================
pub fn restart_def() -> CommandDef {
    CommandDef {
        name: "restart",
        about: "Restart service containers",
        needs_project: true,
        args: vec![
            ArgDef::Flag {
                id: "NO_DEPS",
                long: "no-deps",
                short: None,
                help: "Don't restart dependent services",
            },
            ArgDef::Number {
                id: "TIMEOUT",
                long: "timeout",
                short: Some('t'),
                help: "Specify a shutdown timeout in seconds",
            },
            ArgDef::Services,
        ],
    }
}

// ============================================================================
// docker compose rm
// https://docs.docker.com/reference/cli/docker/compose/rm/
// ============================================================================
pub fn rm_def() -> CommandDef {
    CommandDef {
        name: "rm",
        about: "Removes stopped service containers",
        needs_project: true,
        args: vec![
            ArgDef::Flag {
                id: "FORCE",
                long: "force",
                short: Some('f'),
                help: "Don't ask to confirm removal",
            },
            ArgDef::Flag {
                id: "STOP",
                long: "stop",
                short: Some('s'),
                help: "Stop the containers, if required, before removing",
            },
            ArgDef::Flag {
                id: "VOLUMES",
                long: "volumes",
                short: Some('v'),
                help: "Remove any anonymous volumes attached to containers",
            },
            ArgDef::Services,
        ],
    }
}

// ============================================================================
// docker compose run
// https://docs.docker.com/reference/cli/docker/compose/run/
// ============================================================================
pub fn run_def() -> CommandDef {
    CommandDef {
        name: "run",
        about: "Run a one-off command on a service",
        needs_project: true,
        args: vec![
            ArgDef::Flag {
                id: "BUILD",
                long: "build",
                short: None,
                help: "Build image before starting container",
            },
            ArgDef::Value {
                id: "CAP_ADD",
                long: "cap-add",
                short: None,
                help: "Add Linux capabilities",
            },
            ArgDef::Value {
                id: "CAP_DROP",
                long: "cap-drop",
                short: None,
                help: "Drop Linux capabilities",
            },
            ArgDef::Flag {
                id: "DETACH",
                long: "detach",
                short: Some('d'),
                help: "Run container in background and print container ID",
            },
            ArgDef::Value {
                id: "ENTRYPOINT",
                long: "entrypoint",
                short: None,
                help: "Override the entrypoint of the image",
            },
            ArgDef::Value {
                id: "ENV",
                long: "env",
                short: Some('e'),
                help: "Set environment variables",
            },
            ArgDef::Value {
                id: "ENV_FROM_FILE",
                long: "env-from-file",
                short: None,
                help: "Set environment variables from file",
            },
            ArgDef::Flag {
                id: "INTERACTIVE",
                long: "interactive",
                short: Some('i'),
                help: "Keep STDIN open even if not attached",
            },
            ArgDef::Value {
                id: "LABEL",
                long: "label",
                short: Some('l'),
                help: "Add or override a label",
            },
            ArgDef::Value {
                id: "NAME",
                long: "name",
                short: None,
                help: "Assign a name to the container",
            },
            ArgDef::Flag {
                id: "NO_TTY",
                long: "no-TTY",
                short: Some('T'),
                help: "Disable pseudo-TTY allocation",
            },
            ArgDef::Flag {
                id: "NO_DEPS",
                long: "no-deps",
                short: None,
                help: "Don't start linked services",
            },
            ArgDef::Value {
                id: "PUBLISH",
                long: "publish",
                short: Some('p'),
                help: "Publish a container's port(s) to the host",
            },
            ArgDef::Choice {
                id: "PULL",
                long: "pull",
                short: None,
                help: "Pull image before running",
                choices: &["always", "missing", "never"],
            },
            ArgDef::Flag {
                id: "QUIET",
                long: "quiet",
                short: Some('q'),
                help: "Don't print anything to STDOUT",
            },
            ArgDef::Flag {
                id: "QUIET_BUILD",
                long: "quiet-build",
                short: None,
                help: "Suppress progress output from the build process",
            },
            ArgDef::Flag {
                id: "QUIET_PULL",
                long: "quiet-pull",
                short: None,
                help: "Pull without printing progress information",
            },
            ArgDef::Flag {
                id: "REMOVE_ORPHANS",
                long: "remove-orphans",
                short: None,
                help: "Remove containers for services not defined in Compose file",
            },
            ArgDef::Flag {
                id: "RM",
                long: "rm",
                short: None,
                help: "Automatically remove the container when it exits",
            },
            ArgDef::Flag {
                id: "SERVICE_PORTS",
                long: "service-ports",
                short: Some('P'),
                help: "Run command with all service's ports enabled and mapped to host",
            },
            ArgDef::Flag {
                id: "USE_ALIASES",
                long: "use-aliases",
                short: None,
                help: "Use the service's network useAliases in connected networks",
            },
            ArgDef::Value {
                id: "USER",
                long: "user",
                short: Some('u'),
                help: "Run as specified username or uid",
            },
            ArgDef::Value {
                id: "VOLUME",
                long: "volume",
                short: Some('v'),
                help: "Bind mount a volume",
            },
            ArgDef::Value {
                id: "WORKDIR",
                long: "workdir",
                short: Some('w'),
                help: "Working directory inside the container",
            },
            ArgDef::ServiceWithCommand,
        ],
    }
}

// ============================================================================
// docker compose start
// https://docs.docker.com/reference/cli/docker/compose/start/
// ============================================================================
pub fn start_def() -> CommandDef {
    CommandDef {
        name: "start",
        about: "Start services",
        needs_project: true,
        args: vec![
            ArgDef::Flag {
                id: "WAIT",
                long: "wait",
                short: None,
                help: "Wait for services to be running|healthy",
            },
            ArgDef::Number {
                id: "WAIT_TIMEOUT",
                long: "wait-timeout",
                short: None,
                help: "Maximum duration to wait for the project to be running|healthy",
            },
            ArgDef::Services,
        ],
    }
}

// ============================================================================
// docker compose stop
// https://docs.docker.com/reference/cli/docker/compose/stop/
// ============================================================================
pub fn stop_def() -> CommandDef {
    CommandDef {
        name: "stop",
        about: "Stop services",
        needs_project: true,
        args: vec![
            ArgDef::Number {
                id: "TIMEOUT",
                long: "timeout",
                short: Some('t'),
                help: "Specify a shutdown timeout in seconds",
            },
            ArgDef::Services,
        ],
    }
}

// ============================================================================
// docker compose top
// https://docs.docker.com/reference/cli/docker/compose/top/
// ============================================================================
pub fn top_def() -> CommandDef {
    CommandDef {
        name: "top",
        about: "Display the running processes",
        needs_project: true,
        args: vec![ArgDef::Services],
    }
}

// ============================================================================
// docker compose unpause
// https://docs.docker.com/reference/cli/docker/compose/unpause/
// ============================================================================
pub fn unpause_def() -> CommandDef {
    CommandDef {
        name: "unpause",
        about: "Unpause services",
        needs_project: true,
        args: vec![ArgDef::Services],
    }
}

// ============================================================================
// docker compose up
// https://docs.docker.com/reference/cli/docker/compose/up/
// ============================================================================
pub fn up_def() -> CommandDef {
    CommandDef {
        name: "up",
        about: "Create and start containers",
        needs_project: true,
        args: vec![
            ArgDef::Flag {
                id: "ABORT_ON_CONTAINER_EXIT",
                long: "abort-on-container-exit",
                short: None,
                help: "Stops all containers if any container was stopped",
            },
            ArgDef::Flag {
                id: "ABORT_ON_CONTAINER_FAILURE",
                long: "abort-on-container-failure",
                short: None,
                help: "Stops all containers if any container exited with failure",
            },
            ArgDef::Flag {
                id: "ALWAYS_RECREATE_DEPS",
                long: "always-recreate-deps",
                short: None,
                help: "Recreate dependent containers",
            },
            ArgDef::Value {
                id: "ATTACH",
                long: "attach",
                short: None,
                help: "Restrict attaching to the specified services",
            },
            ArgDef::Flag {
                id: "ATTACH_DEPENDENCIES",
                long: "attach-dependencies",
                short: None,
                help: "Automatically attach to log output of dependent services",
            },
            ArgDef::Flag {
                id: "BUILD",
                long: "build",
                short: None,
                help: "Build images before starting containers",
            },
            ArgDef::Flag {
                id: "DETACH",
                long: "detach",
                short: Some('d'),
                help: "Detached mode: Run containers in the background",
            },
            ArgDef::Value {
                id: "EXIT_CODE_FROM",
                long: "exit-code-from",
                short: None,
                help: "Return the exit code of the selected service container",
            },
            ArgDef::Flag {
                id: "FORCE_RECREATE",
                long: "force-recreate",
                short: None,
                help: "Recreate containers even if their configuration and image haven't changed",
            },
            ArgDef::Flag {
                id: "MENU",
                long: "menu",
                short: None,
                help: "Enable interactive shortcuts when running attached",
            },
            ArgDef::Value {
                id: "NO_ATTACH",
                long: "no-attach",
                short: None,
                help: "Do not attach (stream logs) to the specified services",
            },
            ArgDef::Flag {
                id: "NO_BUILD",
                long: "no-build",
                short: None,
                help: "Don't build an image, even if it's missing",
            },
            ArgDef::Flag {
                id: "NO_COLOR",
                long: "no-color",
                short: None,
                help: "Produce monochrome output",
            },
            ArgDef::Flag {
                id: "NO_DEPS",
                long: "no-deps",
                short: None,
                help: "Don't start linked services",
            },
            ArgDef::Flag {
                id: "NO_LOG_PREFIX",
                long: "no-log-prefix",
                short: None,
                help: "Don't print prefix in logs",
            },
            ArgDef::Flag {
                id: "NO_RECREATE",
                long: "no-recreate",
                short: None,
                help: "If containers already exist, don't recreate them",
            },
            ArgDef::Flag {
                id: "NO_START",
                long: "no-start",
                short: None,
                help: "Don't start the services after creating them",
            },
            ArgDef::Choice {
                id: "PULL",
                long: "pull",
                short: None,
                help: "Pull image before running",
                choices: &["always", "missing", "never"],
            },
            ArgDef::Flag {
                id: "QUIET_BUILD",
                long: "quiet-build",
                short: None,
                help: "Suppress progress output from the build process",
            },
            ArgDef::Flag {
                id: "QUIET_PULL",
                long: "quiet-pull",
                short: None,
                help: "Pull without printing progress information",
            },
            ArgDef::Flag {
                id: "REMOVE_ORPHANS",
                long: "remove-orphans",
                short: None,
                help: "Remove containers for services not defined in the Compose file",
            },
            ArgDef::Flag {
                id: "RENEW_ANON_VOLUMES",
                long: "renew-anon-volumes",
                short: Some('V'),
                help: "Recreate anonymous volumes instead of retrieving data from the previous containers",
            },
            ArgDef::Value {
                id: "SCALE",
                long: "scale",
                short: None,
                help: "Scale SERVICE to NUM instances",
            },
            ArgDef::Number {
                id: "TIMEOUT",
                long: "timeout",
                short: Some('t'),
                help: "Use this timeout in seconds for container shutdown",
            },
            ArgDef::Flag {
                id: "TIMESTAMPS",
                long: "timestamps",
                short: None,
                help: "Show timestamps",
            },
            ArgDef::Flag {
                id: "WAIT",
                long: "wait",
                short: None,
                help: "Wait for services to be running|healthy",
            },
            ArgDef::Number {
                id: "WAIT_TIMEOUT",
                long: "wait-timeout",
                short: None,
                help: "Maximum duration to wait for the project to be running|healthy",
            },
            ArgDef::Flag {
                id: "WATCH",
                long: "watch",
                short: Some('w'),
                help: "Watch source code and rebuild/refresh containers when files are updated",
            },
            ArgDef::Flag {
                id: "YES",
                long: "yes",
                short: Some('y'),
                help: "Assume 'yes' as answer to all prompts",
            },
            ArgDef::Services,
        ],
    }
}

// ============================================================================
// docker compose watch
// https://docs.docker.com/reference/cli/docker/compose/watch/
// ============================================================================
pub fn watch_def() -> CommandDef {
    CommandDef {
        name: "watch",
        about: "Watch build context for service and rebuild/refresh containers when files are updated",
        needs_project: true,
        args: vec![
            ArgDef::Flag {
                id: "NO_UP",
                long: "no-up",
                short: None,
                help: "Do not build & start services before watching",
            },
            ArgDef::Flag {
                id: "PRUNE",
                long: "prune",
                short: None,
                help: "Prune dangling images on rebuild",
            },
            ArgDef::Flag {
                id: "QUIET",
                long: "quiet",
                short: None,
                help: "Hide build output",
            },
            ArgDef::Services,
        ],
    }
}

// ============================================================================
// docker compose config
// https://docs.docker.com/reference/cli/docker/compose/config/
// ============================================================================
pub fn config_def() -> CommandDef {
    CommandDef {
        name: "config",
        about: "Parse, resolve and render compose file in canonical format",
        needs_project: true,
        args: vec![
            ArgDef::Choice {
                id: "FORMAT",
                long: "format",
                short: None,
                help: "Format the output (yaml or json)",
                choices: &["yaml", "json"],
            },
            ArgDef::Flag {
                id: "QUIET",
                long: "quiet",
                short: Some('q'),
                help: "Only validate the configuration, don't print anything",
            },
            ArgDef::Flag {
                id: "NO_CONSISTENCY",
                long: "no-consistency",
                short: None,
                help: "Don't check model consistency",
            },
            ArgDef::Flag {
                id: "NO_INTERPOLATE",
                long: "no-interpolate",
                short: None,
                help: "Don't interpolate environment variables",
            },
            ArgDef::Flag {
                id: "NO_NORMALIZE",
                long: "no-normalize",
                short: None,
                help: "Don't normalize compose model",
            },
            ArgDef::Flag {
                id: "NO_PATH_RESOLUTION",
                long: "no-path-resolution",
                short: None,
                help: "Don't resolve file paths",
            },
            ArgDef::Flag {
                id: "RESOLVE_IMAGE_DIGESTS",
                long: "resolve-image-digests",
                short: None,
                help: "Pin image tags to digests",
            },
            ArgDef::Value {
                id: "OUTPUT",
                long: "output",
                short: Some('o'),
                help: "Save to file (default to stdout)",
            },
            ArgDef::Flag {
                id: "HASH",
                long: "hash",
                short: None,
                help: "Print the service config hash",
            },
            ArgDef::Flag {
                id: "IMAGES",
                long: "images",
                short: None,
                help: "Print the image names",
            },
            ArgDef::Flag {
                id: "PROFILES",
                long: "profiles",
                short: None,
                help: "Print the profile names",
            },
            ArgDef::Flag {
                id: "SERVICES",
                long: "services",
                short: None,
                help: "Print the service names",
            },
            ArgDef::Flag {
                id: "VOLUMES",
                long: "volumes",
                short: None,
                help: "Print the volume names",
            },
            ArgDef::Services,
        ],
    }
}

// ============================================================================
// docker compose attach
// https://docs.docker.com/reference/cli/docker/compose/attach/
// ============================================================================
pub fn attach_def() -> CommandDef {
    CommandDef {
        name: "attach",
        about: "Attach local standard input, output, and error streams to a service's running container",
        needs_project: true,
        args: vec![
            ArgDef::Value {
                id: "DETACH_KEYS",
                long: "detach-keys",
                short: None,
                help: "Override the key sequence for detaching from a container",
            },
            ArgDef::Number {
                id: "INDEX",
                long: "index",
                short: None,
                help: "Index of the container if service has multiple replicas",
            },
            ArgDef::Flag {
                id: "NO_STDIN",
                long: "no-stdin",
                short: None,
                help: "Do not attach STDIN",
            },
            ArgDef::Flag {
                id: "SIG_PROXY",
                long: "sig-proxy",
                short: None,
                help: "Proxy all received signals to the process",
            },
            ArgDef::Container,
        ],
    }
}

// ============================================================================
// docker compose bridge
// https://docs.docker.com/reference/cli/docker/compose/bridge/
// ============================================================================
pub fn bridge_def() -> CommandDef {
    CommandDef {
        name: "bridge",
        about: "Convert compose files into another model",
        needs_project: true,
        args: vec![ArgDef::ServiceWithCommand],
    }
}

// ============================================================================
// docker compose commit
// https://docs.docker.com/reference/cli/docker/compose/commit/
// ============================================================================
pub fn commit_def() -> CommandDef {
    CommandDef {
        name: "commit",
        about: "Create a new image from a service container's changes",
        needs_project: true,
        args: vec![
            ArgDef::Value {
                id: "AUTHOR",
                long: "author",
                short: Some('a'),
                help: "Author (e.g., \"John Hannibal Smith <hannibal@a-team.com>\")",
            },
            ArgDef::Value {
                id: "CHANGE",
                long: "change",
                short: Some('c'),
                help: "Apply Dockerfile instruction to the created image",
            },
            ArgDef::Number {
                id: "INDEX",
                long: "index",
                short: None,
                help: "Index of the container if service has multiple replicas",
            },
            ArgDef::Value {
                id: "MESSAGE",
                long: "message",
                short: Some('m'),
                help: "Commit message",
            },
            ArgDef::Flag {
                id: "PAUSE",
                long: "pause",
                short: Some('p'),
                help: "Pause container during commit",
            },
            ArgDef::ServiceWithCommand,
        ],
    }
}

// ============================================================================
// docker compose cp
// https://docs.docker.com/reference/cli/docker/compose/cp/
// ============================================================================
pub fn cp_def() -> CommandDef {
    CommandDef {
        name: "cp",
        about: "Copy files/folders between a service container and the local filesystem",
        needs_project: true,
        args: vec![
            ArgDef::Flag {
                id: "ALL",
                long: "all",
                short: None,
                help: "Include containers created by the run command",
            },
            ArgDef::Flag {
                id: "ARCHIVE",
                long: "archive",
                short: Some('a'),
                help: "Archive mode (copy all uid/gid information)",
            },
            ArgDef::Flag {
                id: "FOLLOW_LINK",
                long: "follow-link",
                short: Some('L'),
                help: "Always follow symbol link in SRC_PATH",
            },
            ArgDef::Number {
                id: "INDEX",
                long: "index",
                short: None,
                help: "Index of the container if service has multiple replicas",
            },
            ArgDef::ServiceWithCommand,
        ],
    }
}

// ============================================================================
// docker compose export
// https://docs.docker.com/reference/cli/docker/compose/export/
// ============================================================================
pub fn export_def() -> CommandDef {
    CommandDef {
        name: "export",
        about: "Export a service container's filesystem as a tar archive",
        needs_project: true,
        args: vec![
            ArgDef::Number {
                id: "INDEX",
                long: "index",
                short: None,
                help: "Index of the container if service has multiple replicas",
            },
            ArgDef::Value {
                id: "OUTPUT",
                long: "output",
                short: Some('o'),
                help: "Write to a file, instead of STDOUT",
            },
            ArgDef::Container,
        ],
    }
}

// ============================================================================
// docker compose publish
// https://docs.docker.com/reference/cli/docker/compose/publish/
// ============================================================================
pub fn publish_def() -> CommandDef {
    CommandDef {
        name: "publish",
        about: "Publish compose application",
        needs_project: true,
        args: vec![
            ArgDef::Flag {
                id: "APP",
                long: "app",
                short: None,
                help: "Published compose application (includes referenced images)",
            },
            ArgDef::Value {
                id: "OCI_VERSION",
                long: "oci-version",
                short: None,
                help: "OCI image/artifact specification version",
            },
            ArgDef::Flag {
                id: "RESOLVE_IMAGE_DIGESTS",
                long: "resolve-image-digests",
                short: None,
                help: "Pin image tags to digests",
            },
            ArgDef::Flag {
                id: "WITH_ENV",
                long: "with-env",
                short: None,
                help: "Include environment variables in the published OCI artifact",
            },
            ArgDef::Flag {
                id: "YES",
                long: "yes",
                short: Some('y'),
                help: "Assume 'yes' as answer to all prompts",
            },
            ArgDef::Container,
        ],
    }
}

// ============================================================================
// docker compose scale
// https://docs.docker.com/reference/cli/docker/compose/scale/
// ============================================================================
pub fn scale_def() -> CommandDef {
    CommandDef {
        name: "scale",
        about: "Scale services",
        needs_project: true,
        args: vec![
            ArgDef::Flag {
                id: "NO_DEPS",
                long: "no-deps",
                short: None,
                help: "Don't start linked services",
            },
            ArgDef::Services,
        ],
    }
}

// ============================================================================
// docker compose stats
// https://docs.docker.com/reference/cli/docker/compose/stats/
// ============================================================================
pub fn stats_def() -> CommandDef {
    CommandDef {
        name: "stats",
        about: "Display a live stream of container(s) resource usage statistics",
        needs_project: true,
        args: vec![
            ArgDef::Flag {
                id: "ALL",
                long: "all",
                short: Some('a'),
                help: "Show all containers (default shows just running)",
            },
            ArgDef::Value {
                id: "FORMAT",
                long: "format",
                short: None,
                help: "Format output using a custom template",
            },
            ArgDef::Flag {
                id: "NO_STREAM",
                long: "no-stream",
                short: None,
                help: "Disable streaming stats and only pull the first result",
            },
            ArgDef::Flag {
                id: "NO_TRUNC",
                long: "no-trunc",
                short: None,
                help: "Do not truncate output",
            },
            ArgDef::Services,
        ],
    }
}

// ============================================================================
// docker compose version
// https://docs.docker.com/reference/cli/docker/compose/version/
// ============================================================================
pub fn version_def() -> CommandDef {
    CommandDef {
        name: "version",
        about: "Show the Docker Compose version information",
        needs_project: false,
        args: vec![
            ArgDef::Choice {
                id: "FORMAT",
                long: "format",
                short: Some('f'),
                help: "Format the output",
                choices: &["pretty", "json"],
            },
            ArgDef::Flag {
                id: "SHORT",
                long: "short",
                short: None,
                help: "Shows only Compose's version number",
            },
        ],
    }
}

// ============================================================================
// docker compose volumes
// https://docs.docker.com/reference/cli/docker/compose/volumes/
// ============================================================================
pub fn volumes_def() -> CommandDef {
    CommandDef {
        name: "volumes",
        about: "List volumes",
        needs_project: true,
        args: vec![
            ArgDef::Value {
                id: "FORMAT",
                long: "format",
                short: None,
                help: "Format output using a custom template",
            },
            ArgDef::Flag {
                id: "QUIET",
                long: "quiet",
                short: Some('q'),
                help: "Only display volume names",
            },
            ArgDef::Services,
        ],
    }
}

// ============================================================================
// docker compose wait
// https://docs.docker.com/reference/cli/docker/compose/wait/
// ============================================================================
pub fn wait_def() -> CommandDef {
    CommandDef {
        name: "wait",
        about: "Block until containers of all (or specified) services stop",
        needs_project: true,
        args: vec![
            ArgDef::Flag {
                id: "DOWN_PROJECT",
                long: "down-project",
                short: None,
                help: "Drops project when the first container stops",
            },
            ArgDef::Services,
        ],
    }
}

/// Get all command definitions
pub fn all_definitions() -> Vec<CommandDef> {
    vec![
        attach_def(),
        bridge_def(),
        build_def(),
        commit_def(),
        config_def(),
        cp_def(),
        create_def(),
        down_def(),
        events_def(),
        exec_def(),
        export_def(),
        images_def(),
        kill_def(),
        logs_def(),
        ls_def(),
        pause_def(),
        port_def(),
        ps_def(),
        publish_def(),
        pull_def(),
        push_def(),
        restart_def(),
        rm_def(),
        run_def(),
        scale_def(),
        start_def(),
        stats_def(),
        stop_def(),
        top_def(),
        unpause_def(),
        up_def(),
        version_def(),
        volumes_def(),
        wait_def(),
        watch_def(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_definitions_count() {
        assert_eq!(all_definitions().len(), 35);
    }

    #[test]
    fn test_build_command_has_correct_args() {
        let def = build_def();
        assert_eq!(def.name, "build");
        assert!(def.needs_project);
        // Should have Services arg
        assert!(def.args.iter().any(|a| matches!(a, ArgDef::Services)));
    }

    #[test]
    fn test_up_command_has_correct_args() {
        let def = up_def();
        assert_eq!(def.name, "up");
        // Should have PULL as Choice (not Flag)
        assert!(def.args.iter().any(|a| matches!(a, ArgDef::Choice { id: "PULL", .. })));
        // Should have TIMEOUT as Number
        assert!(def.args.iter().any(|a| matches!(a, ArgDef::Number { id: "TIMEOUT", .. })));
    }

    #[test]
    fn test_down_command_has_correct_args() {
        let def = down_def();
        assert_eq!(def.name, "down");
        // Should have RMI as Choice
        assert!(def.args.iter().any(|a| matches!(a, ArgDef::Choice { id: "RMI", .. })));
    }

    #[test]
    fn test_ls_does_not_need_project() {
        let def = ls_def();
        assert!(!def.needs_project);
    }

    #[test]
    fn test_all_definitions_build_valid_commands() {
        for def in all_definitions() {
            let cmd = def.to_clap_command();
            assert_eq!(cmd.get_name(), def.name);
        }
    }
}
