use std::error::Error;
use clap::Subcommand;
use tabled::{builder::Builder, settings::Style};
use crate::common::Execute;

#[derive(Subcommand)]
pub(crate) enum LoggerCommand {
    #[clap(about = "List loggers")]
    List {
        #[clap(short, long, help = "Filter loggers by this")]
        filter: Option<String>,

        #[clap(short, long, help = "Order loggers by this")]
        order_by: Option<String>,
    }
}

impl Execute for LoggerCommand {
    fn execute(&self, api: wikijs::Api) -> Result<(), Box<dyn Error>> {
        match self {
            LoggerCommand::List { filter, order_by } => {
                logger_list(api, filter.to_owned(), order_by.to_owned())
            }
        }
    }
}

fn logger_list(api: wikijs::Api, filter: Option<String>, order_by: Option<String>) -> Result<(), Box<dyn Error>> {
    let loggers = api.logger_list(filter, order_by)?;
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
    Ok(())
}
