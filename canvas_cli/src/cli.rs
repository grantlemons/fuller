use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The auth token to use for requests.
    /// This overrides the default in the configuration.
    /// If neither is set, the OAuth2 process will be used.
    #[arg(long, value_name = "TOKEN")]
    pub token: Option<String>,

    /// Sets a custom config file.
    #[arg(long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Course subcommands
    /// Lists a user's courses by default
    Courses {
        #[command(subcommand)]
        command: Option<CoursesCommands>,
    },
    /// Todo subcommands
    /// Lists a user's todo list by default
    Todo {
        #[command(subcommand)]
        command: Option<TodoCommands>,
    },
    /// Inbox subcommands
    /// Views a user's inbox contents by default
    Inbox {
        #[command(subcommand)]
        command: Option<InboxCommands>,
    },
    /// Profile subcommands
    /// Views a user's profile by default
    Profile {
        #[command(subcommand)]
        command: Option<ProfileCommands>,
    },
}

#[derive(Subcommand)]
pub enum CoursesCommands {}

#[derive(Subcommand)]
pub enum TodoCommands {}

#[derive(Subcommand)]
pub enum InboxCommands {}

#[derive(Subcommand)]
pub enum ProfileCommands {}
