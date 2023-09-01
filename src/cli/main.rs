use clap::{Parser, Subcommand};
use colored::Colorize;
use tabled::{builder::Builder, settings::Style};
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

    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[clap(about = "Page commands")]
    Page {
        #[clap(subcommand)]
        command: PageCommand,
    },
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
}

fn main() {
    let cli = Cli::parse();
    let credentials = Credentials::Key(cli.key);
    let api = Api::new(cli.url, credentials);

    match cli.command {
        Command::Page { command } => match command {
            PageCommand::Get { id } => match api.get_page(id) {
                Ok(page) => {
                    let mut builder = Builder::new();
                    builder.push_record(["key", "value"]);
                    builder.push_record(["id", page.id.to_string().as_str()]);
                    builder.push_record(["path", page.path.to_string().as_str()]);
                    builder.push_record(["hash", page.hash.to_string().as_str()]);
                    builder.push_record(["title", page.title.as_str()]);
                    // TODO description
                    builder.push_record(["is_private", page.is_private.to_string().as_str()]);
                    builder.push_record(["is_published", page.is_published.to_string().as_str()]);
                    builder.push_record([
                        "private_ns",
                        page.private_ns.unwrap_or("".to_string()).as_str(),
                    ]);
                    builder
                        .push_record(["publish_start_date", &page.publish_start_date.to_string()]);
                    builder.push_record(["publish_end_date", &page.publish_end_date.to_string()]);
                    // TODO tags
                    // TODO content
                    // TODO toc
                    // TODO render
                    builder.push_record(["content_type", page.content_type.as_str()]);
                    builder.push_record(["created_at", &page.created_at.to_string()]);
                    builder.push_record(["updated_at", &page.updated_at.to_string()]);
                    builder.push_record(["editor", page.editor.as_str()]);
                    builder.push_record(["locale", page.locale.as_str()]);
                    // TODO script_css
                    // TODO script_js
                    builder.push_record(["author_id", page.author_id.to_string().as_str()]);
                    builder.push_record(["author_name", page.author_name.as_str()]);
                    builder.push_record(["author_email", page.author_email.as_str()]);
                    builder.push_record(["creator_id", page.creator_id.to_string().as_str()]);
                    builder.push_record(["creator_name", page.creator_name.as_str()]);
                    builder.push_record(["creator_email", page.creator_email.as_str()]);
                    println!("{}", builder.build().with(Style::rounded()));
                }
                Err(e) => eprintln!("{}: {}", "Error".bold().red(), e.to_string()),
            },
            PageCommand::List {} => match api.list_all_pages() {
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
                Err(e) => eprintln!("{}: {}", "Error".bold().red(), e.to_string()),
            },
        },
    }
}
