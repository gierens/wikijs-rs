use crate::common::Execute;
use clap::Subcommand;
use std::error::Error;
use tabled::{builder::Builder, settings::Style};

#[derive(Subcommand, Debug)]
pub(crate) enum AuthenticationStrategyCommand {
    #[clap(about = "List authentication strategies")]
    List {},
}

impl Execute for AuthenticationStrategyCommand {
    fn execute(&self, api: wikijs::Api) -> Result<(), Box<dyn Error>> {
        match self {
            AuthenticationStrategyCommand::List {} => authentication_strategy_list(api),
        }
    }
}

fn authentication_strategy_list(api: wikijs::Api) -> Result<(), Box<dyn Error>> {
    let providers = api.authentication_strategy_list()?;
    let mut builder = Builder::new();
    builder.push_record([
        "key",
        // "props",
        "title",
        // "description",
        "is_available",
        // "use_form",
        // "username_type",
        // "logo",
        // "color",
        // "website",
        // "icon",
    ]);
    for provider in providers {
        builder.push_record([
            provider.key.as_str(),
            // provider.props.as_str(),
            provider.title.as_str(),
            // provider.description.as_str(),
            match provider.is_available {
                Some(true) => "true",
                Some(false) => "false",
                None => "",
            },
            // provider.use_form.to_string().as_str(),
            // provider.username_type.as_str(),
            // provider.logo.as_str(),
            // provider.color.as_str(),
            // provider.website.as_str(),
            // provider.icon.as_str(),
        ]);
    }
    println!("{}", builder.build().with(Style::rounded()));
    Ok(())
}
