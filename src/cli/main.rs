use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use wikijs::{Api, Credentials};

mod analytics;
mod asset;
mod authentication;
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

#[derive(Args, Debug)]
#[group(required = true, multiple = true)]
struct CredentialArgs {
    #[clap(short, long, help = "Wiki.js API key", env = "WIKI_JS_API_KEY")]
    key: Option<String>,

    #[clap(
        short = 'U',
        long,
        help = "Wiki.js username",
        env = "WIKI_JS_USERNAME",
        requires = "password",
        conflicts_with = "key"
    )]
    username: Option<String>,

    #[clap(
        short = 'P',
        long,
        help = "Wiki.js password",
        env = "WIKI_JS_PASSWORD",
        requires = "username",
        conflicts_with = "key"
    )]
    password: Option<String>,

    #[clap(
        short,
        long,
        help = "Wiki.js authentication provider ID",
        env = "WIKI_JS_AUTH_PROVIDER",
        default_value = "local"
    )]
    provider: Option<String>,
}

#[derive(Parser, Debug)]
#[command(name = "wikijs-cli")]
#[command(author = "Sandro-Alessio Gierens <sandro@gierens.de>")]
#[command(version = "0.1.1")]
#[command(about = "Command line client for Wiki.js")]
struct Cli {
    #[clap(short, long, help = "Wiki.js base URL", env = "WIKI_JS_BASE_URL")]
    url: String,

    #[clap(flatten)]
    credentials: CredentialArgs,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
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

    #[clap(about = "Authentication strategy commands")]
    AuthenticationStrategy {
        #[clap(subcommand)]
        command: authentication::AuthenticationStrategyCommand,
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
    let credentials = match cli.credentials.key {
        Some(key) => Credentials::Key(key),
        None => {
            let username = cli.credentials.username.unwrap();
            let password = cli.credentials.password.unwrap();
            let provider = cli.credentials.provider.unwrap();
            Credentials::UsernamePassword(username, password, provider)
        }
    };
    let api = Api::new(cli.url.clone(), credentials).unwrap_or_else(|e| {
        eprintln!("{}: {}", "error".bold().red(), e);
        std::process::exit(1);
    });

    // TODO each command should be in its own module
    // TODO each subcommand should implement an Execute trait to call here

    match match cli.command {
        Command::Asset { ref command } => command.execute(api),
        Command::AssetFolder { ref command } => command.execute(api),
        Command::AuthenticationStrategy { ref command } => command.execute(api),
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
