use crate::common::Execute;
use clap::Subcommand;
use std::error::Error;
use tabled::{builder::Builder, settings::Style};

#[derive(Subcommand)]
pub(crate) enum SystemFlagCommand {
    #[clap(about = "List system flags")]
    List {},
}

impl Execute for SystemFlagCommand {
    fn execute(&self, api: wikijs::Api) -> Result<(), Box<dyn Error>> {
        match self {
            SystemFlagCommand::List {} => system_flag_list(api),
        }
    }
}

fn system_flag_list(api: wikijs::Api) -> Result<(), Box<dyn Error>> {
    let flags = api.system_flag_list()?;
    let mut builder = Builder::new();
    builder.push_record(["key", "value"]);
    for flag in flags {
        builder
            .push_record([flag.key.as_str(), flag.value.to_string().as_str()]);
    }
    println!("{}", builder.build().with(Style::rounded()));
    Ok(())
}
