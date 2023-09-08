use clap::Subcommand;
use colored::Colorize;
use tabled::{builder::Builder, settings::Style};
use crate::common::Execute;

#[derive(Subcommand)]
pub(crate) enum AssetCommand {
    #[clap(about = "List assets")]
    List {},
}

#[derive(Subcommand)]
pub(crate) enum AssetFolderCommand {
    #[clap(about = "List asset folders")]
    List {
        #[clap(help = "Parent folder ID")]
        parent_folder_id: i64,
    },
}

impl Execute for AssetCommand {
    fn execute(&self, api: wikijs::Api) {
        match self {
            AssetCommand::List {} => asset_list(api),
        }
    }
}

impl Execute for AssetFolderCommand {
    fn execute(&self, api: wikijs::Api) {
        match self {
            AssetFolderCommand::List { parent_folder_id } => {
                asset_folder_list(api, *parent_folder_id)
            }
        }
    }
}

fn asset_list(api: wikijs::Api) {
    match api.asset_list(0, wikijs::asset::AssetKind::ALL) {
        Ok(assets) => {
            let mut builder = Builder::new();
            builder.push_record([
                "id",
                "filename",
                "ext",
                "kind",
                "mime",
                "file_size",
                "metadata",
                "created_at",
                "updated_at",
                // "folder",
                // "author",
            ]);
            for asset in assets {
                builder.push_record([
                    asset.id.to_string().as_str(),
                    asset.filename.as_str(),
                    asset.ext.as_str(),
                    asset.kind.to_string().as_str(),
                    asset.mime.as_str(),
                    asset.file_size.to_string().as_str(),
                    asset.metadata.unwrap_or("".to_string()).as_str(),
                    asset.created_at.to_string().as_str(),
                    asset.updated_at.to_string().as_str(),
                    // TODO
                    // asset.folder.to_string().as_str(),
                    // asset.author.unwrap_or(0).to_string().as_str(),
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

fn asset_folder_list(api: wikijs::Api, parent_folder_id: i64) {
    match api.asset_folder_list(parent_folder_id) {
        Ok(asset_folders) => {
            let mut builder = Builder::new();
            builder.push_record([
                "id",
                "slug",
                "name",
            ]);
            for asset_folder in asset_folders {
                builder.push_record([
                    asset_folder.id.to_string().as_str(),
                    asset_folder.slug.as_str(),
                    asset_folder
                        .name
                        .unwrap_or("".to_string())
                        .as_str(),
                ]);
            }
            println!(
                "{}",
                builder.build().with(Style::rounded())
            );
        }
        Err(e) => {
            eprintln!("{}: {}", "error".bold().red(), e.to_string());
            std::process::exit(1);
        }
    }
}
