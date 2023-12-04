use crate::common::Execute;
use clap::Subcommand;
use std::error::Error;
use tabled::{builder::Builder, settings::Style};

#[derive(Subcommand, Debug)]
pub(crate) enum CommentCommand {
    #[clap(about = "List comments")]
    List {
        #[clap(short, long, help = "Page locale", default_value = "en")]
        locale: String,

        #[clap(help = "Page path")]
        path: String,
    },
}

impl Execute for CommentCommand {
    fn execute(&self, api: wikijs::Api) -> Result<(), Box<dyn Error>> {
        match self {
            CommentCommand::List { locale, path } => {
                comment_list(api, locale.to_string(), path.to_string())
            }
        }
    }
}

fn comment_list(
    api: wikijs::Api,
    locale: String,
    path: String,
) -> Result<(), Box<dyn Error>> {
    let comments = api.comment_list(locale, path)?;
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
    Ok(())
}
