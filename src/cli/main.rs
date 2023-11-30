use clap::{Parser, Subcommand};
use colored::Colorize;
use wikijs::{Api, Credentials};

mod analytics;
mod asset;
mod comment;
mod common;
mod contribute;
mod group;
mod localization;
mod logger;
mod page;
mod system;
mod theming;
mod user;

use crate::common::Execute;

#[derive(Parser)]
#[command(name = "wikijs-cli")]
#[command(author = "Sandro-Alessio Gierens <sandro@gierens.de>")]
#[command(version = "0.1.1")]
#[command(about = "Command line client for Wiki.js")]
struct Cli {
    #[clap(short, long, help = "Wiki.js base URL", env = "WIKI_JS_BASE_URL")]
    url: String,

    #[clap(short, long, help = "Wiki.js API key", env = "WIKI_JS_API_KEY")]
    key: String,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[clap(about = "Asset commands")]
    Asset {
        #[clap(subcommand)]
        command: asset::AssetCommand,
    },

    #[clap(about = "Asset folder commands")]
    AssetFolder {
        #[clap(subcommand)]
        command: asset::AssetFolderCommand,
    },

    #[clap(about = "Page commands")]
    Page {
        #[clap(subcommand)]
        command: page::PageCommand,
    },

    #[clap(about = "Contributor commands")]
    Contributor {
        #[clap(subcommand)]
        command: contribute::ContributorCommand,
    },

    #[clap(about = "Analytics provider commands")]
    AnalyticsProvider {
        #[clap(subcommand)]
        command: analytics::AnalyticsProviderCommand,
    },

    #[clap(about = "Comment commands")]
    Comment {
        #[clap(subcommand)]
        command: comment::CommentCommand,
    },

    #[clap(about = "User commands")]
    User {
        #[clap(subcommand)]
        command: user::UserCommand,
    },

    #[clap(about = "User profile commands")]
    Profile {
        #[clap(subcommand)]
        command: user::ProfileCommand,
    },

    #[clap(about = "Password commands")]
    Password {
        #[clap(subcommand)]
        command: user::PasswordCommand,
    },

    #[clap(about = "Group commands")]
    Group {
        #[clap(subcommand)]
        command: group::GroupCommand,
    },

    #[clap(about = "Locale commands")]
    Locale {
        #[clap(subcommand)]
        command: localization::LocaleCommand,
    },

    #[clap(about = "Logger commands")]
    Logger {
        #[clap(subcommand)]
        command: logger::LoggerCommand,
    },

    #[clap(about = "System flag commands")]
    SystemFlag {
        #[clap(subcommand)]
        command: system::SystemFlagCommand,
    },

    #[clap(about = "Theme commands")]
    Theme {
        #[clap(subcommand)]
        command: theming::ThemeCommand,
    },
}

fn main() {
    let cli = Cli::parse();
    let credentials = Credentials::Key(cli.key.clone());
    let api = Api::new(cli.url.clone(), credentials).unwrap_or_else(|e| {
        eprintln!("{}: {}", "error".bold().red(), e);
        std::process::exit(1);
    });

    // TODO each command should be in its own module
    // TODO each subcommand should implement an Execute trait to call here

    match match cli.command {
        Command::Asset { ref command } => command.execute(api),
        Command::AssetFolder { ref command } => command.execute(api),
        Command::Page { ref command } => command.execute(api),
        Command::Contributor { ref command } => command.execute(api),
        Command::AnalyticsProvider { command } => command.execute(api),
        Command::Comment { ref command } => command.execute(api),
        Command::User { ref command } => command.execute(api),
        Command::Profile { ref command } => command.execute(api),
        Command::Password { ref command } => command.execute(api),
        Command::Group { command } => command.execute(api),
        Command::Locale { command } => command.execute(api),
        Command::Logger { command } => command.execute(api),
        Command::SystemFlag { command } => command.execute(api),
        Command::Theme { command } => command.execute(api),
    } {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}: {}", "error".bold().red(), e);
            std::process::exit(1);
        }
    }
}
