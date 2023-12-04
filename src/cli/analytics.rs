use crate::common::Execute;
use clap::Subcommand;
use std::error::Error;
use tabled::{builder::Builder, settings::Style};

#[derive(Subcommand, Debug)]
pub(crate) enum AnalyticsProviderCommand {
    #[clap(about = "List analytics providers")]
    List {},
}

impl Execute for AnalyticsProviderCommand {
    fn execute(&self, api: wikijs::Api) -> Result<(), Box<dyn Error>> {
        match self {
            AnalyticsProviderCommand::List {} => analytics_provider_list(api),
        }
    }
}

fn analytics_provider_list(api: wikijs::Api) -> Result<(), Box<dyn Error>> {
    let providers = api.analytics_provider_list()?;
    let mut builder = Builder::new();
    builder.push_record([
        "is_enabled",
        "key",
        // "props",
        "title",
        // "description",
        // "is_available",
        // "logo",
        // "website",
        // "config",
    ]);
    for provider in providers {
        builder.push_record([
            provider.is_enabled.to_string().as_str(),
            provider.key.as_str(),
            // provider.props.as_str(),
            provider.title.as_str(),
            // provider.description.as_str(),
            // provider.is_available.to_string().as_str(),
            // provider.logo.as_str(),
            // provider.website.as_str(),
            // provider.config.as_str(),
        ]);
    }
    println!("{}", builder.build().with(Style::rounded()));
    Ok(())
}
