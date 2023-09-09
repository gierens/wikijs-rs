use clap::Subcommand;
use colored::Colorize;
use tabled::{builder::Builder, settings::Style};
use crate::common::Execute;

#[derive(Subcommand)]
pub(crate) enum GroupCommand {
    #[clap(about = "List groups")]
    List {
        #[clap(short, long, help = "Filter groups by this")]
        filter: Option<String>,

        #[clap(short, long, help = "Order groups by this")]
        order_by: Option<String>,
    },
}

impl Execute for GroupCommand {
    fn execute(&self, api: wikijs::Api) {
        match self {
            GroupCommand::List { filter, order_by } => {
                group_list(api, filter.to_owned(), order_by.to_owned())
            }
        }
    }
}

fn group_list(api: wikijs::Api, filter: Option<String>, order_by: Option<String>) {
    match api.group_list(filter, order_by) {
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
}
