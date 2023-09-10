use crate::common::Execute;
use clap::Subcommand;
use std::error::Error;
use tabled::{builder::Builder, settings::Style};

#[derive(Subcommand)]
pub(crate) enum LocaleCommand {
    #[clap(about = "List locales")]
    List,
}

impl Execute for LocaleCommand {
    fn execute(&self, api: wikijs::Api) -> Result<(), Box<dyn Error>> {
        match self {
            LocaleCommand::List => locale_list(api),
        }
    }
}

fn locale_list(api: wikijs::Api) -> Result<(), Box<dyn Error>> {
    let locales = api.locale_list()?;
    let mut builder = Builder::new();
    builder.push_record([
        "availability",
        "code",
        "created_at",
        "install_date",
        "is_installed",
        "is_rtl",
        "name",
        "native_name",
        "updated_at",
    ]);
    for locale in locales {
        builder.push_record([
            locale.availability.to_string().as_str(),
            locale.code.as_str(),
            locale.created_at.to_string().as_str(),
            locale.install_date.unwrap_or("".to_string()).as_str(),
            locale.is_installed.to_string().as_str(),
            locale.is_rtl.to_string().as_str(),
            locale.name.as_str(),
            locale.native_name.as_str(),
            locale.updated_at.to_string().as_str(),
        ]);
    }
    println!("{}", builder.build().with(Style::rounded()));
    Ok(())
}
