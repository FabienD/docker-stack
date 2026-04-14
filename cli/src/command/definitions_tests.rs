//! Comprehensive tests for all docker compose command definitions
//!
//! Each test validates that:
//! 1. The command can be parsed by clap
//! 2. Arguments are correctly extracted
//! 3. The output matches expected docker compose format

#[cfg(test)]
mod tests {
    use std::ffi::OsString;
    use crate::command::definitions::*;

    // ========================================================================
    // Helper function to compare OsString vectors
    // ========================================================================
    fn assert_args_eq(actual: Vec<OsString>, expected: Vec<&str>) {
        let expected: Vec<OsString> = expected.into_iter().map(OsString::from).collect();
        assert_eq!(actual, expected, "Arguments mismatch");
    }

    // ========================================================================
    // docker compose build
    // ========================================================================
    #[test]
    fn test_build_minimal() {
        let def = build_def();
        let matches = def.to_clap_command().get_matches_from(vec!["build", "myproject"]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["build"]);
    }

    #[test]
    fn test_build_with_flags() {
        let def = build_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "build", "--no-cache", "--pull", "--quiet", "myproject"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["build", "--no-cache", "--pull", "--quiet"]);
    }

    #[test]
    fn test_build_with_progress_choice() {
        let def = build_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "build", "--progress", "plain", "myproject"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["build", "--progress", "plain"]);
    }

    #[test]
    fn test_build_with_services() {
        let def = build_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "build", "--no-cache", "myproject", "web", "api"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["build", "--no-cache", "web", "api"]);
    }

    #[test]
    fn test_build_with_value_args() {
        let def = build_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "build", "--build-arg", "VERSION=1.0", "--ssh", "default", "myproject"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["build", "--build-arg", "VERSION=1.0", "--ssh", "default"]);
    }

    // ========================================================================
    // docker compose create
    // ========================================================================
    #[test]
    fn test_create_minimal() {
        let def = create_def();
        let matches = def.to_clap_command().get_matches_from(vec!["create", "myproject"]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["create"]);
    }

    #[test]
    fn test_create_with_flags() {
        let def = create_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "create", "--build", "--force-recreate", "--remove-orphans", "myproject"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["create", "--build", "--force-recreate", "--remove-orphans"]);
    }

    #[test]
    fn test_create_with_pull_choice() {
        let def = create_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "create", "--pull", "always", "myproject"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["create", "--pull", "always"]);
    }

    // ========================================================================
    // docker compose down
    // ========================================================================
    #[test]
    fn test_down_minimal() {
        let def = down_def();
        let matches = def.to_clap_command().get_matches_from(vec!["down", "myproject"]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["down"]);
    }

    #[test]
    fn test_down_with_all_options() {
        let def = down_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "down", "--remove-orphans", "--rmi", "local", "--timeout", "30", "--volumes", "myproject"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["down", "--remove-orphans", "--rmi", "local", "--timeout", "30", "--volumes"]);
    }

    #[test]
    fn test_down_rmi_all() {
        let def = down_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "down", "--rmi", "all", "myproject"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["down", "--rmi", "all"]);
    }

    // ========================================================================
    // docker compose events
    // ========================================================================
    #[test]
    fn test_events_minimal() {
        let def = events_def();
        let matches = def.to_clap_command().get_matches_from(vec!["events", "myproject"]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["events"]);
    }

    #[test]
    fn test_events_with_json() {
        let def = events_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "events", "--json", "myproject"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["events", "--json"]);
    }

    // ========================================================================
    // docker compose exec
    // ========================================================================
    #[test]
    fn test_exec_minimal() {
        let def = exec_def();
        let matches = def.to_clap_command().get_matches_from(vec!["exec", "myproject", "web"]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["exec", "web"]);
    }

    #[test]
    fn test_exec_with_options() {
        let def = exec_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "exec", "-d", "-T", "--user", "root", "--workdir", "/app", "myproject", "web"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["exec", "--detach", "--no-TTY", "--user", "root", "--workdir", "/app", "web"]);
    }

    #[test]
    fn test_exec_with_command() {
        // Test case: dctl exec myproject php bash
        let def = exec_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "exec", "myproject", "php", "bash"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["exec", "php", "bash"]);
    }

    #[test]
    fn test_exec_with_command_and_args() {
        // Test case: dctl exec myproject php bash -c "echo hello"
        let def = exec_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "exec", "myproject", "php", "bash", "-c", "echo hello"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["exec", "php", "bash", "-c", "echo hello"]);
    }

    #[test]
    fn test_exec_with_options_and_command() {
        // Test case: dctl exec -T myproject php bin/console cache:clear
        let def = exec_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "exec", "-T", "myproject", "php", "bin/console", "cache:clear"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["exec", "--no-TTY", "php", "bin/console", "cache:clear"]);
    }

    // ========================================================================
    // docker compose images
    // ========================================================================
    #[test]
    fn test_images_minimal() {
        let def = images_def();
        let matches = def.to_clap_command().get_matches_from(vec!["images", "myproject"]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["images"]);
    }

    #[test]
    fn test_images_with_format() {
        let def = images_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "images", "--format", "json", "-q", "myproject"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["images", "--format", "json", "--quiet"]);
    }

    // ========================================================================
    // docker compose kill
    // ========================================================================
    #[test]
    fn test_kill_minimal() {
        let def = kill_def();
        let matches = def.to_clap_command().get_matches_from(vec!["kill", "myproject"]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["kill"]);
    }

    #[test]
    fn test_kill_with_signal() {
        let def = kill_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "kill", "-s", "SIGTERM", "myproject"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["kill", "--signal", "SIGTERM"]);
    }

    // ========================================================================
    // docker compose logs
    // ========================================================================
    #[test]
    fn test_logs_minimal() {
        let def = logs_def();
        let matches = def.to_clap_command().get_matches_from(vec!["logs", "myproject"]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["logs"]);
    }

    #[test]
    fn test_logs_with_all_options() {
        let def = logs_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "logs", "-f", "--no-color", "--no-log-prefix", "--since", "1h",
            "--tail", "100", "-t", "myproject", "web"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec![
            "logs", "--follow", "--no-color", "--no-log-prefix",
            "--since", "1h", "--tail", "100", "--timestamps", "web"
        ]);
    }

    // ========================================================================
    // docker compose ls
    // ========================================================================
    #[test]
    fn test_ls_minimal() {
        let def = ls_def();
        // ls doesn't need a project
        let matches = def.to_clap_command().get_matches_from(vec!["ls"]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["ls"]);
    }

    #[test]
    fn test_ls_with_options() {
        let def = ls_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "ls", "-a", "--format", "json", "-q"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["ls", "--all", "--format", "json", "--quiet"]);
    }

    // ========================================================================
    // docker compose pause
    // ========================================================================
    #[test]
    fn test_pause_minimal() {
        let def = pause_def();
        let matches = def.to_clap_command().get_matches_from(vec!["pause", "myproject"]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["pause"]);
    }

    #[test]
    fn test_pause_with_services() {
        let def = pause_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "pause", "myproject", "web", "db"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["pause", "web", "db"]);
    }

    // ========================================================================
    // docker compose port
    // ========================================================================
    #[test]
    fn test_port_minimal() {
        let def = port_def();
        let matches = def.to_clap_command().get_matches_from(vec!["port", "myproject", "web"]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["port", "web"]);
    }

    #[test]
    fn test_port_with_protocol() {
        let def = port_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "port", "--protocol", "udp", "myproject", "web"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["port", "--protocol", "udp", "web"]);
    }

    // ========================================================================
    // docker compose ps
    // ========================================================================
    #[test]
    fn test_ps_minimal() {
        let def = ps_def();
        let matches = def.to_clap_command().get_matches_from(vec!["ps", "myproject"]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["ps"]);
    }

    #[test]
    fn test_ps_with_options() {
        let def = ps_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "ps", "-a", "--format", "json", "-q", "--status", "running", "myproject"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["ps", "--all", "--format", "json", "--quiet", "--status", "running"]);
    }

    // ========================================================================
    // docker compose pull
    // ========================================================================
    #[test]
    fn test_pull_minimal() {
        let def = pull_def();
        let matches = def.to_clap_command().get_matches_from(vec!["pull", "myproject"]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["pull"]);
    }

    #[test]
    fn test_pull_with_options() {
        let def = pull_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "pull", "--ignore-pull-failures", "--include-deps", "--policy", "always", "-q", "myproject"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec![
            "pull", "--ignore-pull-failures", "--include-deps", "--policy", "always", "--quiet"
        ]);
    }

    // ========================================================================
    // docker compose push
    // ========================================================================
    #[test]
    fn test_push_minimal() {
        let def = push_def();
        let matches = def.to_clap_command().get_matches_from(vec!["push", "myproject"]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["push"]);
    }

    #[test]
    fn test_push_with_options() {
        let def = push_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "push", "--ignore-push-failures", "--include-deps", "-q", "myproject"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["push", "--ignore-push-failures", "--include-deps", "--quiet"]);
    }

    // ========================================================================
    // docker compose restart
    // ========================================================================
    #[test]
    fn test_restart_minimal() {
        let def = restart_def();
        let matches = def.to_clap_command().get_matches_from(vec!["restart", "myproject"]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["restart"]);
    }

    #[test]
    fn test_restart_with_timeout() {
        let def = restart_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "restart", "--no-deps", "-t", "30", "myproject", "web"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["restart", "--no-deps", "--timeout", "30", "web"]);
    }

    // ========================================================================
    // docker compose rm
    // ========================================================================
    #[test]
    fn test_rm_minimal() {
        let def = rm_def();
        let matches = def.to_clap_command().get_matches_from(vec!["rm", "myproject"]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["rm"]);
    }

    #[test]
    fn test_rm_with_options() {
        let def = rm_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "rm", "-f", "-s", "-v", "myproject"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["rm", "--force", "--stop", "--volumes"]);
    }

    // ========================================================================
    // docker compose run
    // ========================================================================
    #[test]
    fn test_run_minimal() {
        let def = run_def();
        let matches = def.to_clap_command().get_matches_from(vec!["run", "myproject", "web"]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["run", "web"]);
    }

    #[test]
    fn test_run_with_common_options() {
        let def = run_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "run", "-d", "--rm", "--no-deps", "-u", "root", "-w", "/app", "myproject", "web"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec![
            "run", "--detach", "--no-deps", "--rm", "--user", "root", "--workdir", "/app", "web"
        ]);
    }

    #[test]
    fn test_run_with_command() {
        // Test case: dctl run myproject php composer install
        let def = run_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "run", "myproject", "php", "composer", "install"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["run", "php", "composer", "install"]);
    }

    #[test]
    fn test_run_with_options_and_command() {
        // Test case: dctl run --rm myproject php bin/console cache:clear --env=dev
        let def = run_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "run", "--rm", "myproject", "php", "bin/console", "cache:clear", "--env=dev"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["run", "--rm", "php", "bin/console", "cache:clear", "--env=dev"]);
    }

    // ========================================================================
    // docker compose start
    // ========================================================================
    #[test]
    fn test_start_minimal() {
        let def = start_def();
        let matches = def.to_clap_command().get_matches_from(vec!["start", "myproject"]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["start"]);
    }

    #[test]
    fn test_start_with_wait() {
        let def = start_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "start", "--wait", "--wait-timeout", "60", "myproject", "web"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["start", "--wait", "--wait-timeout", "60", "web"]);
    }

    // ========================================================================
    // docker compose stop
    // ========================================================================
    #[test]
    fn test_stop_minimal() {
        let def = stop_def();
        let matches = def.to_clap_command().get_matches_from(vec!["stop", "myproject"]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["stop"]);
    }

    #[test]
    fn test_stop_with_timeout() {
        let def = stop_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "stop", "-t", "30", "myproject", "web", "db"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["stop", "--timeout", "30", "web", "db"]);
    }

    // ========================================================================
    // docker compose top
    // ========================================================================
    #[test]
    fn test_top_minimal() {
        let def = top_def();
        let matches = def.to_clap_command().get_matches_from(vec!["top", "myproject"]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["top"]);
    }

    #[test]
    fn test_top_with_services() {
        let def = top_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "top", "myproject", "web", "db"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["top", "web", "db"]);
    }

    // ========================================================================
    // docker compose unpause
    // ========================================================================
    #[test]
    fn test_unpause_minimal() {
        let def = unpause_def();
        let matches = def.to_clap_command().get_matches_from(vec!["unpause", "myproject"]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["unpause"]);
    }

    #[test]
    fn test_unpause_with_services() {
        let def = unpause_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "unpause", "myproject", "web", "db"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["unpause", "web", "db"]);
    }

    // ========================================================================
    // docker compose up
    // ========================================================================
    #[test]
    fn test_up_minimal() {
        let def = up_def();
        let matches = def.to_clap_command().get_matches_from(vec!["up", "myproject"]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["up"]);
    }

    #[test]
    fn test_up_detached() {
        let def = up_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "up", "-d", "--remove-orphans", "myproject"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["up", "--detach", "--remove-orphans"]);
    }

    #[test]
    fn test_up_with_build() {
        let def = up_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "up", "--build", "--force-recreate", "myproject"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["up", "--build", "--force-recreate"]);
    }

    #[test]
    fn test_up_with_pull_choice() {
        let def = up_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "up", "--pull", "always", "myproject"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["up", "--pull", "always"]);
    }

    #[test]
    fn test_up_with_timeout() {
        let def = up_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "up", "-t", "60", "--wait", "myproject"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["up", "--timeout", "60", "--wait"]);
    }

    #[test]
    fn test_up_with_services() {
        let def = up_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "up", "-d", "myproject", "web", "api"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["up", "--detach", "web", "api"]);
    }

    #[test]
    fn test_up_complex() {
        let def = up_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "up", "-d", "--build", "--force-recreate", "--remove-orphans",
            "--pull", "always", "-t", "30", "myproject", "web"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec![
            "up", "--build", "--detach", "--force-recreate",
            "--pull", "always", "--remove-orphans", "--timeout", "30", "web"
        ]);
    }

    // ========================================================================
    // docker compose watch
    // ========================================================================
    #[test]
    fn test_watch_minimal() {
        let def = watch_def();
        let matches = def.to_clap_command().get_matches_from(vec!["watch", "myproject"]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["watch"]);
    }

    #[test]
    fn test_watch_with_options() {
        let def = watch_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "watch", "--no-up", "--prune", "--quiet", "myproject"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["watch", "--no-up", "--prune", "--quiet"]);
    }

    // ========================================================================
    // Edge cases and validation tests
    // ========================================================================
    #[test]
    fn test_all_commands_parse_without_error() {
        for def in all_definitions() {
            let cmd = def.to_clap_command();
            // Verify the command builds correctly
            cmd.debug_assert();
        }
    }

    #[test]
    fn test_choice_validation_rejects_invalid() {
        let def = down_def();
        let result = def.to_clap_command().try_get_matches_from(vec![
            "down", "--rmi", "invalid", "myproject"
        ]);
        assert!(result.is_err(), "Should reject invalid choice value");
    }

    #[test]
    fn test_number_validation_rejects_negative() {
        let def = stop_def();
        let result = def.to_clap_command().try_get_matches_from(vec![
            "stop", "-t", "-5", "myproject"
        ]);
        assert!(result.is_err(), "Should reject negative number");
    }

    #[test]
    fn test_number_validation_rejects_non_numeric() {
        let def = stop_def();
        let result = def.to_clap_command().try_get_matches_from(vec![
            "stop", "-t", "abc", "myproject"
        ]);
        assert!(result.is_err(), "Should reject non-numeric value");
    }

    // ========================================================================
    // docker compose attach
    // ========================================================================
    #[test]
    fn test_attach_minimal() {
        let def = attach_def();
        let matches = def.to_clap_command().get_matches_from(vec!["attach", "myproject", "web"]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["attach", "web"]);
    }

    #[test]
    fn test_attach_with_flags() {
        let def = attach_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "attach", "--no-stdin", "--index", "2", "myproject", "web"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["attach", "--index", "2", "--no-stdin", "web"]);
    }

    // ========================================================================
    // docker compose bridge
    // ========================================================================
    #[test]
    fn test_bridge_with_subcommand() {
        let def = bridge_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "bridge", "myproject", "convert"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["bridge", "convert"]);
    }

    // ========================================================================
    // docker compose commit
    // ========================================================================
    #[test]
    fn test_commit_minimal() {
        let def = commit_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "commit", "myproject", "web"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["commit", "web"]);
    }

    #[test]
    fn test_commit_with_message_and_tag() {
        let def = commit_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "commit", "-m", "snapshot", "myproject", "web", "myimage:1.0"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["commit", "--message", "snapshot", "web", "myimage:1.0"]);
    }

    // ========================================================================
    // docker compose config
    // ========================================================================
    #[test]
    fn test_config_minimal() {
        let def = config_def();
        let matches = def.to_clap_command().get_matches_from(vec!["config", "myproject"]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["config"]);
    }

    #[test]
    fn test_config_with_services_flag() {
        let def = config_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "config", "--services", "--quiet", "myproject"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["config", "--quiet", "--services"]);
    }

    #[test]
    fn test_config_with_format_choice() {
        let def = config_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "config", "--format", "json", "myproject"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["config", "--format", "json"]);
    }

    // ========================================================================
    // docker compose cp
    // ========================================================================
    #[test]
    fn test_cp_minimal() {
        let def = cp_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "cp", "myproject", "web:/etc/conf", "./conf"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["cp", "web:/etc/conf", "./conf"]);
    }

    #[test]
    fn test_cp_with_archive_flag() {
        let def = cp_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "cp", "-a", "myproject", "web:/etc/conf", "./conf"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["cp", "--archive", "web:/etc/conf", "./conf"]);
    }

    // ========================================================================
    // docker compose export
    // ========================================================================
    #[test]
    fn test_export_minimal() {
        let def = export_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "export", "myproject", "web"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["export", "web"]);
    }

    #[test]
    fn test_export_with_output() {
        let def = export_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "export", "-o", "web.tar", "myproject", "web"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["export", "--output", "web.tar", "web"]);
    }

    // ========================================================================
    // docker compose publish
    // ========================================================================
    #[test]
    fn test_publish_minimal() {
        let def = publish_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "publish", "myproject", "myregistry/myapp:1.0"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["publish", "myregistry/myapp:1.0"]);
    }

    #[test]
    fn test_publish_with_flags() {
        let def = publish_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "publish", "--with-env", "--resolve-image-digests", "myproject", "myapp:1"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["publish", "--resolve-image-digests", "--with-env", "myapp:1"]);
    }

    // ========================================================================
    // docker compose scale
    // ========================================================================
    #[test]
    fn test_scale_minimal() {
        let def = scale_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "scale", "myproject", "web=3"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["scale", "web=3"]);
    }

    #[test]
    fn test_scale_with_no_deps() {
        let def = scale_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "scale", "--no-deps", "myproject", "web=3", "api=2"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["scale", "--no-deps", "web=3", "api=2"]);
    }

    // ========================================================================
    // docker compose stats
    // ========================================================================
    #[test]
    fn test_stats_minimal() {
        let def = stats_def();
        let matches = def.to_clap_command().get_matches_from(vec!["stats", "myproject"]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["stats"]);
    }

    #[test]
    fn test_stats_with_flags() {
        let def = stats_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "stats", "-a", "--no-stream", "--no-trunc", "myproject"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["stats", "--all", "--no-stream", "--no-trunc"]);
    }

    // ========================================================================
    // docker compose version
    // ========================================================================
    #[test]
    fn test_version_minimal() {
        let def = version_def();
        let matches = def.to_clap_command().get_matches_from(vec!["version"]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["version"]);
    }

    #[test]
    fn test_version_with_format_and_short() {
        let def = version_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "version", "-f", "json", "--short"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["version", "--format", "json", "--short"]);
    }

    #[test]
    fn test_version_does_not_need_project() {
        let def = version_def();
        assert!(!def.needs_project);
    }

    // ========================================================================
    // docker compose volumes
    // ========================================================================
    #[test]
    fn test_volumes_minimal() {
        let def = volumes_def();
        let matches = def.to_clap_command().get_matches_from(vec!["volumes", "myproject"]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["volumes"]);
    }

    #[test]
    fn test_volumes_with_quiet() {
        let def = volumes_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "volumes", "-q", "myproject"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["volumes", "--quiet"]);
    }

    // ========================================================================
    // docker compose wait
    // ========================================================================
    #[test]
    fn test_wait_minimal() {
        let def = wait_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "wait", "myproject", "web"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["wait", "web"]);
    }

    #[test]
    fn test_wait_with_down_project() {
        let def = wait_def();
        let matches = def.to_clap_command().get_matches_from(vec![
            "wait", "--down-project", "myproject", "web", "api"
        ]);
        let args = def.prepare_args(&matches);
        assert_args_eq(args, vec!["wait", "--down-project", "web", "api"]);
    }
}