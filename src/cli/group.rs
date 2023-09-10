use crate::common::Execute;
use clap::Subcommand;
use std::error::Error;
use tabled::{builder::Builder, settings::Style};

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
    fn execute(&self, api: wikijs::Api) -> Result<(), Box<dyn Error>> {
        match self {
            GroupCommand::List { filter, order_by } => {
                group_list(api, filter.to_owned(), order_by.to_owned())
            }
        }
    }
}

fn group_list(
    api: wikijs::Api,
    filter: Option<String>,
    order_by: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let groups = api.group_list(filter, order_by)?;
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
    Ok(())
}
