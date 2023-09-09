use clap::{Parser, Subcommand};
use colored::Colorize;
use tabled::{builder::Builder, settings::Style};
use wikijs::{Api, Credentials};

mod analytics;
mod asset;
mod comment;
mod common;
mod contribute;
mod group;
mod locale;
mod page;
mod system;
mod theming;
mod user;

use crate::common::Execute;


#[derive(Parser)]
#[command(name = "wikijs-cli")]
#[command(author = "Sandro-Alessio Gierens <sandro@gierens.de>")]
#[command(version = "0.1.0")]
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
        command: AnalyticsProviderCommand,
    },

    #[clap(about = "Comment commands")]
    Comment {
        #[clap(subcommand)]
        command: comment::CommentCommand,
    },

    #[clap(about = "User commands")]
    User {
        #[clap(subcommand)]
        command: UserCommand,
    },

    #[clap(about = "Group commands")]
    Group {
        #[clap(subcommand)]
        command: GroupCommand,
    },

    #[clap(about = "Locale commands")]
    Locale {
        #[clap(subcommand)]
        command: LocaleCommand,
    },

    #[clap(about = "Logger commands")]
    Logger {
        #[clap(subcommand)]
        command: LoggerCommand,
    },

    #[clap(about = "System flag commands")]
    SystemFlag {
        #[clap(subcommand)]
        command: SystemFlagCommand,
    },

    #[clap(about = "Theme commands")]
    Theme {
        #[clap(subcommand)]
        command: ThemeCommand,
    },
}

#[derive(Subcommand)]
enum AnalyticsProviderCommand {
    #[clap(about = "List analytics providers")]
    List {},
}

#[derive(Subcommand)]
enum UserCommand {
    #[clap(about = "Get a user")]
    Get {
        #[clap(help = "User ID")]
        id: i64,
    },

    #[clap(about = "List users")]
    List {
        #[clap(short, long, help = "Filter users by this")]
        filter: Option<String>,

        #[clap(short, long, help = "Order users by this")]
        order_by: Option<String>,
    },
}

#[derive(Subcommand)]
enum GroupCommand {
    #[clap(about = "List groups")]
    List {
        #[clap(short, long, help = "Filter groups by this")]
        filter: Option<String>,

        #[clap(short, long, help = "Order groups by this")]
        order_by: Option<String>,
    },
}

#[derive(Subcommand)]
enum LocaleCommand {
    #[clap(about = "List locales")]
    List

}
#[derive(Subcommand)]
enum LoggerCommand {
    #[clap(about = "List loggers")]
    List {
        #[clap(short, long, help = "Filter loggers by this")]
        filter: Option<String>,

        #[clap(short, long, help = "Order loggers by this")]
        order_by: Option<String>,
    }
}

#[derive(Subcommand)]
enum SystemFlagCommand {
    #[clap(about = "List system flags")]
    List {},
}

#[derive(Subcommand)]
enum ThemeCommand {
    #[clap(about = "List themes")]
    List {},
}

fn main() {
    let cli = Cli::parse();
    let credentials = Credentials::Key(cli.key.clone());
    let api = Api::new(cli.url.clone(), credentials);

    // TODO each command should be in its own module
    // TODO each subcommand should implement an Execute trait to call here

    match cli.command {
        Command::Asset { ref command } => command.execute(api),
        Command::AssetFolder { ref command } => command.execute(api),
        Command::Page { ref command } => command.execute(api),
        Command::Contributor { ref command } => command.execute(api),
        Command::AnalyticsProvider { command } => match command {
            AnalyticsProviderCommand::List {} => {
                match api.analytics_provider_list() {
                    Ok(providers) => {
                        let mut builder = Builder::new();
                        builder.push_record([
                            "is_enabled",
                            "key",
                            // "props",
                            "title",
                            // "description",
                            // "is_available",
                            // "logo",
                            // "website",
                            // "config",
                        ]);
                        for provider in providers {
                            builder.push_record([
                                provider.is_enabled.to_string().as_str(),
                                provider.key.as_str(),
                                // provider.props.as_str(),
                                provider.title.as_str(),
                                // provider.description.as_str(),
                                // provider.is_available.to_string().as_str(),
                                // provider.logo.as_str(),
                                // provider.website.as_str(),
                                // provider.config.as_str(),
                            ]);
                        }
                        println!("{}", builder.build().with(Style::rounded()));
                    }
                    Err(e) => {
                        eprintln!(
                            "{}: {}",
                            "error".bold().red(),
                            e.to_string()
                        );
                        std::process::exit(1);
                    }
                }
            }
        },
        Command::Comment { ref command } => command.execute(api),
        Command::User { command } => match command {
            UserCommand::Get { id } => match api.user_get(id) {
                Ok(user) => {
                    let mut builder = Builder::new();
                    builder.push_record(["key", "value"]);
                    builder.push_record(["id", user.id.to_string().as_str()]);
                    builder.push_record(["name", user.name.as_str()]);
                    builder.push_record(["email", user.email.as_str()]);
                    builder.push_record([
                        "provider_key",
                        user.provider_key.as_str(),
                    ]);
                    builder.push_record([
                        "provider_name",
                        user.provider_name.unwrap_or("".to_string()).as_str(),
                    ]);
                    builder.push_record([
                        "provider_id",
                        user.provider_id.unwrap_or("".to_string()).as_str(),
                    ]);
                    // providerIs2FACapable
                    builder.push_record([
                        "is_system",
                        user.is_system.to_string().as_str(),
                    ]);
                    builder.push_record([
                        "is_active",
                        user.is_active.to_string().as_str(),
                    ]);
                    builder.push_record([
                        "is_verified",
                        user.is_verified.to_string().as_str(),
                    ]);
                    builder.push_record(["location", user.location.as_str()]);
                    builder.push_record(["job_title", user.job_title.as_str()]);
                    builder.push_record(["timezone", user.timezone.as_str()]);
                    builder.push_record([
                        "date_format",
                        user.date_format.as_str(),
                    ]);
                    builder
                        .push_record(["appearance", user.appearance.as_str()]);
                    builder.push_record([
                        "created_at",
                        user.created_at.to_string().as_str(),
                    ]);
                    builder.push_record([
                        "updated_at",
                        user.updated_at.to_string().as_str(),
                    ]);
                    builder.push_record([
                        "last_login_at",
                        user.last_login_at.unwrap_or("".to_string()).as_str(),
                    ]);
                    // tfaIsActive
                    // groups
                    println!("{}", builder.build().with(Style::rounded()));
                }
                Err(e) => {
                    eprintln!("{}: {}", "error".bold().red(), e.to_string());
                    std::process::exit(1);
                }
            },
            UserCommand::List { filter, order_by } => match api.user_list(filter, order_by) {
                Ok(users) => {
                    let mut builder = Builder::new();
                    builder.push_record([
                        "id",
                        "name",
                        "email",
                        "provider_key",
                        "is_system",
                        "is_active",
                        "created_at",
                        "last_login_at",
                    ]);
                    for user in users {
                        builder.push_record([
                            user.id.to_string().as_str(),
                            user.name.as_str(),
                            user.email.as_str(),
                            user.provider_key.as_str(),
                            user.is_system.to_string().as_str(),
                            user.is_active.to_string().as_str(),
                            user.created_at.to_string().as_str(),
                            user.last_login_at.unwrap_or("".to_string()).as_str(),
                        ]);
                    }
                    println!("{}", builder.build().with(Style::rounded()));
                }
                Err(e) => {
                    eprintln!("{}: {}", "error".bold().red(), e.to_string());
                    std::process::exit(1);
                }
            }
        },
        Command::Group { command } => match command {
            GroupCommand::List { filter, order_by } => match api.group_list(filter, order_by) {
                Ok(groups) => {
                    let mut builder = Builder::new();
                    builder.push_record([
                        "id",
                        "name",
                        "is_system",
                        "user_count",
                        "created_at",
                        "updated_at",
                    ]);
                    for group in groups {
                        builder.push_record([
                            group.id.to_string().as_str(),
                            group.name.as_str(),
                            group.is_system.to_string().as_str(),
                            group.user_count.unwrap_or(0).to_string().as_str(),
                            group.created_at.to_string().as_str(),
                            group.updated_at.to_string().as_str(),
                        ]);
                    }
                    println!("{}", builder.build().with(Style::rounded()));
                }
                Err(e) => {
                    eprintln!("{}: {}", "error".bold().red(), e.to_string());
                    std::process::exit(1);
                }
            }
        },
        Command::Locale { command } => match command {
            LocaleCommand::List {} => match api.locale_list() {
                Ok(locales) => {
                    let mut builder = Builder::new();
                    builder.push_record([
                        "availability",
                        "code",
                        "created_at",
                        "install_date",
                        "is_installed",
                        "is_rtl",
                        "name",
                        "native_name",
                        "updated_at"
                    ]);
                    for locale in locales {
                        builder.push_record([
                            locale.availability.to_string().as_str(),
                            locale.code.as_str(),
                            locale.created_at.to_string().as_str(),
                            locale.install_date.unwrap_or("".to_string()).as_str(),
                            locale.is_installed.to_string().as_str(),
                            locale.is_rtl.to_string().as_str(),
                            locale.name.as_str(),
                            locale.native_name.as_str(),
                            locale.updated_at.to_string().as_str(),
                        ]);
                    }
                    println!("{}", builder.build().with(Style::rounded()));
                }
                Err(e) => {
                    eprintln!("{}: {}", "error".bold().red(), e.to_string());
                    std::process::exit(1);
                }
            }
        },
        Command::Logger { command } => match command {
            LoggerCommand::List { filter, order_by } => match api.logger_list(filter, order_by) {
                Ok(loggers) => {
                    let mut builder = Builder::new();
                    builder.push_record([
                        "is_enabled",
                        "key",
                        "title",
                        // "description",
                        // "logo",
                        // "website",
                        "level",
                        // "config",
                    ]);
                    for logger in loggers {
                        builder.push_record([
                            logger.is_enabled.to_string().as_str(),
                            logger.key.as_str(),
                            logger.title.as_str(),
                            // logger.description.as_str(),
                            // logger.logo.as_str(),
                            // logger.website.as_str(),
                            logger.level.unwrap_or("".to_string()).as_str(),
                            // logger.config.as_str(),
                        ]);
                    }
                    println!("{}", builder.build().with(Style::rounded()));
                }
                Err(e) => {
                    eprintln!("{}: {}", "error".bold().red(), e.to_string());
                    std::process::exit(1);
                }
            }
        },
        Command::SystemFlag { command } => match command {
            SystemFlagCommand::List {} => match api.system_flag_list() {
                Ok(flags) => {
                    let mut builder = Builder::new();
                    builder.push_record(["key", "value"]);
                    for flag in flags {
                        builder.push_record([
                            flag.key.as_str(),
                            flag.value.to_string().as_str(),
                        ]);
                    }
                    println!("{}", builder.build().with(Style::rounded()));
                }
                Err(e) => {
                    eprintln!("{}: {}", "error".bold().red(), e.to_string());
                    std::process::exit(1);
                }
            },
        },
        Command::Theme { command } => match command {
            ThemeCommand::List {} => match api.theme_list() {
                Ok(themes) => {
                    let mut builder = Builder::new();
                    builder.push_record([
                        "key",
                        "title",
                        "author",
                    ]);
                    for theme in themes {
                        builder.push_record([
                            theme.key.unwrap_or("".to_string()).as_str(),
                            theme.title.unwrap_or("".to_string()).as_str(),
                            theme.author.unwrap_or("".to_string()).as_str(),
                        ]);
                    }
                    println!("{}", builder.build().with(Style::rounded()));
                }
                Err(e) => {
                    eprintln!(
                        "{}: {}",
                        "error".bold().red(),
                        e.to_string()
                    );
                    std::process::exit(1);
                }
            }
        },
    }
}
