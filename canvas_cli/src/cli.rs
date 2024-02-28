use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The auth token to use for requests.
    /// This overrides the default in the configuration.
    /// If neither is set, the OAuth2 process will be used.
    pub token: Option<String>,

    /// Sets a custom config file.
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Courses {
        #[command(subcommand)]
        command: Option<CoursesCommands>,
    },
    Todo {
        #[command(subcommand)]
        command: Option<TodoCommands>,
    },
    Inbox {
        #[command(subcommand)]
        command: Option<InboxCommands>,
    },
}

#[derive(Subcommand)]
pub enum CoursesCommands {}

#[derive(Subcommand)]
pub enum TodoCommands {}

#[derive(Subcommand)]
pub enum InboxCommands {}
