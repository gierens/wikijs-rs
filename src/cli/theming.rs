use clap::Subcommand;
use colored::Colorize;
use tabled::{builder::Builder, settings::Style};
use crate::common::Execute;

#[derive(Subcommand)]
pub(crate) enum ThemeCommand {
    #[clap(about = "List themes")]
    List {},
}

impl Execute for ThemeCommand {
    fn execute(&self, api: wikijs::Api) {
        match self {
            ThemeCommand::List {} => theme_list(api),
        }
    }
}

pub fn theme_list(api: wikijs::Api) {
    match api.theme_list() {
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
}
