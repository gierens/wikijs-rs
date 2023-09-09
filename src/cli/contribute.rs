use clap::Subcommand;
use colored::Colorize;
use tabled::{builder::Builder, settings::Style};
use crate::common::Execute;

#[derive(Subcommand)]
pub(crate) enum ContributorCommand {
    #[clap(about = "List contributors")]
    List {},
}

impl Execute for ContributorCommand {
    fn execute(&self, api: wikijs::Api) {
        match self {
            ContributorCommand::List {} => contributor_list(api),
        }
    }
}

pub(crate) fn contributor_list(api: wikijs::Api) {
    match api.contributor_list() {
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
    }
}