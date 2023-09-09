use std::error::Error;
use clap::Subcommand;
use tabled::{builder::Builder, settings::Style};
use crate::common::Execute;

#[derive(Subcommand)]
pub(crate) enum ThemeCommand {
    #[clap(about = "List themes")]
    List {},
}

impl Execute for ThemeCommand {
    fn execute(&self, api: wikijs::Api) -> Result<(), Box<dyn Error>> {
        match self {
            ThemeCommand::List {} => theme_list(api),
        }
    }
}

pub fn theme_list(api: wikijs::Api) -> Result<(), Box<dyn Error>> {
    let themes = api.theme_list()?;
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
    Ok(())
}
