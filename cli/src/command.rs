use clap::{ArgMatches, Command};
use std::ffi::OsString;

use crate::utils::docker::CommandType;

/// Trait for handling docker compose commands
/// This trait allows factoring common command handling logic
pub trait CommandHandler {
    /// Returns the command name (e.g., "build", "up", "down")
    fn name(&self) -> &'static str;

    /// Returns the clap Command definition
    fn cli(&self) -> Command;

    /// Returns the CommandType for docker compose execution
    fn command_type(&self) -> CommandType;

    /// Prepares command arguments from ArgMatches
    fn prepare(&self, args: &ArgMatches) -> Vec<OsString>;
}

// Declarative argument definition system
pub mod args;
pub mod definitions;

#[cfg(test)]
mod definitions_tests;

// Non-docker-compose commands
pub mod cd;
pub mod completion;
pub mod config;
pub mod infos;

// Command registry (uses definitions.rs)
pub mod registry;