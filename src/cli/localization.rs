use clap::Subcommand;
use colored::Colorize;
use tabled::{builder::Builder, settings::Style};
use crate::common::Execute;

#[derive(Subcommand)]
pub(crate) enum LocaleCommand {
    #[clap(about = "List locales")]
    List

}

impl Execute for LocaleCommand {
    fn execute(&self, api: wikijs::Api) {
        match self {
            LocaleCommand::List => locale_list(api),
        }
    }
}

fn locale_list(api: wikijs::Api) {
    match api.locale_list() {
        Ok(locales) => {
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
                "updated_at"
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
        }
        Err(e) => {
            eprintln!("{}: {}", "error".bold().red(), e.to_string());
            std::process::exit(1);
        }
    }
}
