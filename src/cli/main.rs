use clap::{Parser, Subcommand};
use colored::Colorize;
use std::io::Write;
use tabled::{builder::Builder, settings::Style};
use tempfile::Builder as TempFileBuilder;
use wikijs::{Api, Credentials};

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

    #[clap(
        short,
        long,
        help = "Editor to use for editing pages",
        default_value = "vi",
        env = "EDITOR"
    )]
    editor: String,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[clap(about = "Asset commands")]
    Asset {
        #[clap(subcommand)]
        command: AssetCommand,
    },

    #[clap(about = "Page commands")]
    Page {
        #[clap(subcommand)]
        command: PageCommand,
    },

    #[clap(about = "Contributor commands")]
    Contributor {
        #[clap(subcommand)]
        command: ContributorCommand,
    },

    #[clap(about = "Analytics provider commands")]
    AnalyticsProvider {
        #[clap(subcommand)]
        command: AnalyticsProviderCommand,
    },

    #[clap(about = "Comment commands")]
    Comment {
        #[clap(subcommand)]
        command: CommentCommand,
    },

    #[clap(about = "User commands")]
    User {
        #[clap(subcommand)]
        command: UserCommand,
    },

    #[clap(about = "System flag commands")]
    SystemFlag {
        #[clap(subcommand)]
        command: SystemFlagCommand,
    },
}

#[derive(Subcommand)]
enum AssetCommand {
    #[clap(about = "List assets")]
    List {},
}

#[derive(Subcommand)]
enum PageCommand {
    #[clap(about = "Get a page")]
    Get {
        #[clap(help = "Page ID")]
        id: i64,
    },

    #[clap(about = "List pages")]
    List {},

    #[clap(about = "Delete a page")]
    Delete {
        #[clap(help = "Page ID")]
        id: i64,
    },

    #[clap(about = "Render a page")]
    Render {
        #[clap(help = "Page ID")]
        id: i64,
    },

    #[clap(about = "Create a page")]
    Create {
        #[clap(short, long, help = "Page content", default_value = "...")]
        content: String,

        #[clap(short, long, help = "Page description", default_value = "")]
        description: String,

        #[clap(short, long, help = "Page editor", default_value = "markdown")]
        editor: String,

        #[clap(
            short = 'p',
            long,
            help = "Page is private",
            default_value = "false"
        )]
        is_private: bool,

        #[clap(
            short = 'P',
            long,
            help = "Page is published",
            default_value = "true"
        )]
        is_published: bool,

        #[clap(short, long, help = "Page locale", default_value = "en")]
        locale: String,

        #[clap(help = "Page path")]
        path: String,

        // #[clap(help = "Page publish start date")]
        // publish_start_date: Option<String>,

        // #[clap(help = "Page publish end date")]
        // publish_end_date: Option<String>,

        // #[clap(help = "Page CSS script")]
        // script_css: Option<String>,

        // #[clap(help = "Page JS script")]
        // script_js: Option<String>,
        #[clap(short = 'T', long, help = "Page tags")]
        tags: Vec<String>,

        #[clap(short, long, help = "Page title")]
        title: Option<String>,
    },

    #[clap(about = "Update a page")]
    Update {
        #[clap(help = "Page ID")]
        id: i64,

        #[clap(short, long, help = "Page content")]
        content: Option<String>,

        #[clap(short, long, help = "Page description")]
        description: Option<String>,

        #[clap(short, long, help = "Page editor")]
        editor: Option<String>,

        #[clap(short = 'P', long, help = "Page is private")]
        is_private: Option<bool>,

        #[clap(short = 'b', long, help = "Page is published")]
        is_published: Option<bool>,

        #[clap(short, long, help = "Page locale")]
        locale: Option<String>,

        #[clap(short, long, help = "Page path")]
        path: Option<String>,

        // #[clap(help = "Page publish start date")]
        // publish_start_date: Option<String>,

        // #[clap(help = "Page publish end date")]
        // publish_end_date: Option<String>,

        // #[clap(help = "Page CSS script")]
        // script_css: Option<String>,

        // #[clap(help = "Page JS script")]
        // script_js: Option<String>,
        #[clap(short = 'T', long, help = "Page tags")]
        tags: Option<Vec<String>>,

        #[clap(
            short,
            long,
            help = "Remove tags",
            action,
            conflicts_with = "tags"
        )]
        no_tags: bool,

        #[clap(short, long, help = "Page title")]
        title: Option<String>,
    },

    #[clap(about = "Update a page's content")]
    UpdateContent {
        #[clap(help = "Page ID")]
        id: i64,

        #[clap(help = "Page content")]
        content: String,
    },

    #[clap(about = "Edit a page")]
    Edit {
        #[clap(help = "Page ID")]
        id: i64,
    },
}

#[derive(Subcommand)]
enum ContributorCommand {
    #[clap(about = "List contributors")]
    List {},
}

#[derive(Subcommand)]
enum AnalyticsProviderCommand {
    #[clap(about = "List analytics providers")]
    List {},
}

#[derive(Subcommand)]
enum CommentCommand {
    #[clap(about = "List comments")]
    List {
        #[clap(short, long, help = "Page locale", default_value = "en")]
        locale: String,

        #[clap(help = "Page path")]
        path: String,
    },
}

#[derive(Subcommand)]
enum UserCommand {
    #[clap(about = "Get a user")]
    Get {
        #[clap(help = "User ID")]
        id: i64,
    },
}

#[derive(Subcommand)]
enum SystemFlagCommand {
    #[clap(about = "List system flags")]
    List {},
}

fn main() {
    let cli = Cli::parse();
    let credentials = Credentials::Key(cli.key);
    let api = Api::new(cli.url, credentials);

    match cli.command {
        Command::Asset { command } => match command {
            AssetCommand::List {} => match api
                .asset_list(0, wikijs::asset::AssetKind::ALL)
            {
                Ok(assets) => {
                    let mut builder = Builder::new();
                    builder.push_record([
                        "id",
                        "filename",
                        "ext",
                        "kind",
                        "mime",
                        "file_size",
                        "metadata",
                        "created_at",
                        "updated_at",
                        // "folder",
                        // "author",
                    ]);
                    for asset in assets {
                        builder.push_record([
                            asset.id.to_string().as_str(),
                            asset.filename.as_str(),
                            asset.ext.as_str(),
                            asset.kind.to_string().as_str(),
                            asset.mime.as_str(),
                            asset.file_size.to_string().as_str(),
                            asset.metadata.unwrap_or("".to_string()).as_str(),
                            asset.created_at.to_string().as_str(),
                            asset.updated_at.to_string().as_str(),
                            // TODO
                            // asset.folder.to_string().as_str(),
                            // asset.author.unwrap_or(0).to_string().as_str(),
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
        Command::Page { command } => match command {
            PageCommand::Get { id } => match api.page_get(id) {
                Ok(page) => {
                    let mut builder = Builder::new();
                    builder.push_record(["key", "value"]);
                    builder.push_record(["id", page.id.to_string().as_str()]);
                    builder
                        .push_record(["path", page.path.to_string().as_str()]);
                    builder
                        .push_record(["hash", page.hash.to_string().as_str()]);
                    builder.push_record(["title", page.title.as_str()]);
                    // TODO description
                    builder.push_record([
                        "is_private",
                        page.is_private.to_string().as_str(),
                    ]);
                    builder.push_record([
                        "is_published",
                        page.is_published.to_string().as_str(),
                    ]);
                    builder.push_record([
                        "private_ns",
                        page.private_ns.unwrap_or("".to_string()).as_str(),
                    ]);
                    builder.push_record([
                        "publish_start_date",
                        &page.publish_start_date.to_string(),
                    ]);
                    builder.push_record([
                        "publish_end_date",
                        &page.publish_end_date.to_string(),
                    ]);
                    // TODO tags
                    // TODO content
                    // TODO toc
                    // TODO render
                    builder.push_record([
                        "content_type",
                        page.content_type.as_str(),
                    ]);
                    builder.push_record([
                        "created_at",
                        &page.created_at.to_string(),
                    ]);
                    builder.push_record([
                        "updated_at",
                        &page.updated_at.to_string(),
                    ]);
                    builder.push_record(["editor", page.editor.as_str()]);
                    builder.push_record(["locale", page.locale.as_str()]);
                    // TODO script_css
                    // TODO script_js
                    builder.push_record([
                        "author_id",
                        page.author_id.to_string().as_str(),
                    ]);
                    builder.push_record([
                        "author_name",
                        page.author_name.as_str(),
                    ]);
                    builder.push_record([
                        "author_email",
                        page.author_email.as_str(),
                    ]);
                    builder.push_record([
                        "creator_id",
                        page.creator_id.to_string().as_str(),
                    ]);
                    builder.push_record([
                        "creator_name",
                        page.creator_name.as_str(),
                    ]);
                    builder.push_record([
                        "creator_email",
                        page.creator_email.as_str(),
                    ]);
                    println!("{}", builder.build().with(Style::rounded()));
                }
                Err(e) => {
                    eprintln!("{}: {}", "error".bold().red(), e.to_string());
                    std::process::exit(1);
                }
            },
            PageCommand::List {} => match api.page_list() {
                Ok(pages) => {
                    let mut builder = Builder::new();
                    builder.push_record([
                        "id",
                        "locate",
                        "path",
                        "title",
                        "content_type",
                        "is_published",
                        "is_private",
                        "private_ns",
                        "created_at",
                        "updated_at",
                    ]);
                    for page in pages {
                        builder.push_record([
                            page.id.to_string().as_str(),
                            page.path.as_str(),
                            page.locale.as_str(),
                            page.title.unwrap_or("".to_string()).as_str(),
                            // TODO description
                            page.content_type.as_str(),
                            page.is_published.to_string().as_str(),
                            page.is_private.to_string().as_str(),
                            page.private_ns.unwrap_or("".to_string()).as_str(),
                            page.created_at.to_string().as_str(),
                            page.updated_at.to_string().as_str(),
                            // TODO tags
                        ]);
                    }
                    println!("{}", builder.build().with(Style::rounded()));
                }
                Err(e) => {
                    eprintln!("{}: {}", "error".bold().red(), e.to_string());
                    std::process::exit(1);
                }
            },
            PageCommand::Delete { id } => match api.page_delete(id) {
                Ok(_) => {
                    println!("{}: {}", "success".bold().green(), "Page deleted")
                }
                Err(e) => {
                    eprintln!("{}: {}", "error".bold().red(), e.to_string());
                    std::process::exit(1);
                }
            },
            PageCommand::Render { id } => match api.page_render(id) {
                Ok(_) => {
                    println!(
                        "{}: {}",
                        "success".bold().green(),
                        "Page rendered"
                    )
                }
                Err(e) => {
                    eprintln!("{}: {}", "error".bold().red(), e.to_string());
                    std::process::exit(1);
                }
            },
            PageCommand::Create {
                content,
                description,
                editor,
                is_private,
                is_published,
                locale,
                path,
                // publish_start_date,
                // publish_end_date,
                // script_css,
                // script_js,
                tags,
                title,
            } => match api.page_create(
                content,
                description,
                editor,
                is_published,
                is_private,
                locale,
                path.clone(),
                None,
                None,
                None,
                None,
                tags.iter().map(|s| Some(s.clone())).collect(),
                title.unwrap_or(path.split("/").last().unwrap().to_string()),
            ) {
                Ok(()) => {
                    println!("{}: {}", "success".bold().green(), "Page created")
                }
                Err(e) => {
                    eprintln!("{}: {}", "error".bold().red(), e.to_string());
                    std::process::exit(1);
                }
            },
            PageCommand::Update {
                id,
                content,
                description,
                editor,
                is_private,
                is_published,
                locale,
                path,
                // publish_start_date,
                // publish_end_date,
                // script_css,
                // script_js,
                tags,
                no_tags,
                title,
            } => match api.page_update(
                id,
                content,
                description,
                editor,
                is_published,
                is_private,
                locale,
                path,
                None,
                None,
                None,
                None,
                if no_tags {
                    Some(Vec::new())
                } else {
                    match tags {
                        Some(tags) => Some(
                            tags.iter()
                                .map(|s| Some(s.clone()))
                                .collect::<Vec<Option<String>>>(),
                        ),
                        None => None,
                    }
                },
                title,
            ) {
                Ok(()) => {
                    println!("{}: {}", "success".bold().green(), "Page updated")
                }
                Err(e) => {
                    eprintln!("{}: {}", "error".bold().red(), e.to_string());
                    std::process::exit(1);
                }
            },
            PageCommand::UpdateContent { id, content } => {
                match api.page_update_content(id, content) {
                    Ok(()) => {
                        println!(
                            "{}: {}",
                            "success".bold().green(),
                            "Page content updated"
                        )
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
            PageCommand::Edit { id } => {
                let page = match api.page_get(id) {
                    Ok(page) => page,
                    Err(e) => {
                        eprintln!(
                            "{}: {}",
                            "error".bold().red(),
                            e.to_string()
                        );
                        std::process::exit(1);
                    }
                };
                let file = match page.editor.as_str() {
                    "markdown" => TempFileBuilder::new()
                        .suffix(".md")
                        .tempfile()
                        .unwrap_or_else(|e| {
                            eprintln!(
                                "{}: {}",
                                "error".bold().red(),
                                e.to_string()
                            );
                            std::process::exit(1);
                        }),
                    _ => {
                        TempFileBuilder::new().tempfile().unwrap_or_else(|e| {
                            eprintln!(
                                "{}: {}",
                                "error".bold().red(),
                                e.to_string()
                            );
                            std::process::exit(1);
                        })
                    }
                };
                file.reopen()
                    .unwrap_or_else(|e| {
                        eprintln!(
                            "{}: {}",
                            "error".bold().red(),
                            e.to_string()
                        );
                        std::process::exit(1);
                    })
                    .write_all(page.content.as_bytes())
                    .unwrap_or_else(|e| {
                        eprintln!(
                            "{}: {}",
                            "error".bold().red(),
                            e.to_string()
                        );
                        std::process::exit(1);
                    });
                let mut child = std::process::Command::new(cli.editor)
                    .arg(file.path())
                    .spawn()
                    .unwrap_or_else(|e| {
                        eprintln!(
                            "{}: {}",
                            "error".bold().red(),
                            e.to_string()
                        );
                        std::process::exit(1);
                    });
                let status = child.wait().unwrap_or_else(|e| {
                    eprintln!("{}: {}", "error".bold().red(), e.to_string());
                    std::process::exit(1);
                });
                if !status.success() {
                    eprintln!(
                        "{}: {}",
                        "error".bold().red(),
                        "Editor exited with non-zero status"
                    );
                    std::process::exit(1);
                }
                let content = std::fs::read_to_string(file.path())
                    .unwrap_or_else(|e| {
                        eprintln!(
                            "{}: {}",
                            "error".bold().red(),
                            e.to_string()
                        );
                        std::process::exit(1);
                    });
                match api.page_update_content(id, content) {
                    Ok(()) => {
                        println!(
                            "{}: {}",
                            "success".bold().green(),
                            "Page content updated"
                        )
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
        Command::Contributor { command } => match command {
            ContributorCommand::List {} => match api.contributor_list() {
                Ok(contributors) => {
                    let mut builder = Builder::new();
                    builder.push_record([
                        "id", "source", "name",
                        "joined",
                        // "website",
                        // "twitter",
                        // "avatar",
                    ]);
                    for contributor in contributors {
                        builder.push_record([
                            contributor.id.to_string().as_str(),
                            contributor.source.as_str(),
                            contributor.name.as_str(),
                            contributor.joined.as_str(),
                            // TODO these are too long
                            // contributor.website.unwrap_or("".to_string()).as_str(),
                            // contributor.twitter.unwrap_or("".to_string()).as_str(),
                            // contributor.avatar.unwrap_or("".to_string()).as_str(),
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
        Command::Comment { command } => match command {
            CommentCommand::List { locale, path } => {
                match api.comment_list(locale, path) {
                    Ok(comments) => {
                        let mut builder = Builder::new();
                        builder.push_record([
                            "id",
                            // "content",
                            // "render",
                            "author_id",
                            "author_name",
                            "author_email",
                            // "author_ip",
                            "created_at",
                            "updated_at",
                        ]);
                        for comment in comments {
                            builder.push_record([
                                comment.id.to_string().as_str(),
                                // comment.content.as_str(),
                                // comment.render.as_str(),
                                comment.author_id.to_string().as_str(),
                                comment.author_name.as_str(),
                                comment.author_email.as_str(),
                                // comment.author_ip.as_str(),
                                comment.created_at.to_string().as_str(),
                                comment.updated_at.to_string().as_str(),
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
    }
}
