use clap::{Parser, Subcommand};
use fuller_canvas_auth::AccessToken;
use std::path::PathBuf;

#[derive(Parser, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The auth token to use for requests.
    /// This overrides the default in the configuration.
    /// If neither is set, the OAuth2 process will be used.
    #[arg(long, value_name = "TOKEN")]
    pub token: Option<AccessToken>,

    #[arg(long, value_name = "URL")]
    pub url: Option<String>,

    #[arg(long)]
    pub pagination: Option<u16>,

    /// Sets a custom config file.
    #[arg(long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Sets a custom config file.
    #[arg(long)]
    pub no_config: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    /// Course subcommands
    /// Lists a user's courses by default
    Courses {
        #[arg(long, short)]
        course_id: Option<u64>,

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
        #[arg(long, short)]
        inbox_id: Option<u64>,

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

#[derive(Subcommand, Clone, Debug)]
pub enum CoursesCommands {
    Ignore {
        course_ids: Option<Vec<u64>>,
    },
    Assignments {
        assignment_id: Option<u64>,
    },
    Submit {
        assignment_id: Option<u64>,
    },
    Upload {
        path: PathBuf,
        assignment_id: Option<u64>,
    },
}

#[derive(Subcommand, Clone, Debug)]
pub enum TodoCommands {
    Ignore,
}

#[derive(Subcommand, Clone, Debug)]
pub enum InboxCommands {
    Ignore { inbox_ids: Option<Vec<u64>> },
}

#[derive(Subcommand, Clone, Debug)]
pub enum ProfileCommands {}
