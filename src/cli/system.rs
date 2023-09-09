use clap::Subcommand;
use colored::Colorize;
use tabled::{builder::Builder, settings::Style};
use crate::common::Execute;

#[derive(Subcommand)]
pub(crate) enum SystemFlagCommand {
    #[clap(about = "List system flags")]
    List {},
}

impl Execute for SystemFlagCommand {
    fn execute(&self, api: wikijs::Api) {
        match self {
            SystemFlagCommand::List {} => system_flag_list(api),
        }
    }
}

fn system_flag_list(api: wikijs::Api) {
    match api.system_flag_list() {
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
    }
}
