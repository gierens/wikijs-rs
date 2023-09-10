use crate::common::Execute;
use clap::Subcommand;
use colored::Colorize;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use tabled::{builder::Builder, settings::Style};

#[derive(Subcommand)]
pub(crate) enum AssetCommand {
    #[clap(about = "List assets")]
    List {},

    #[clap(about = "Download an asset")]
    Download {
        #[clap(help = "Source path in wiki")]
        source: String,

        #[clap(help = "Destination path on disk")]
        destination: Option<String>,
    },

    #[clap(about = "Upload an asset")]
    Upload {
        #[clap(help = "Source path on disk")]
        source: String,

        #[clap(help = "Destination folder ID")]
        folder: i64,

        #[clap(help = "Destination name in wiki")]
        name: String,
    },
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
    fn execute(&self, api: wikijs::Api) -> Result<(), Box<dyn Error>> {
        match self {
            AssetCommand::List {} => asset_list(api),
            AssetCommand::Download {
                source,
                destination,
            } => asset_download(api, source.to_owned(), destination.to_owned()),
            AssetCommand::Upload {
                source,
                folder,
                name,
            } => asset_upload(
                api,
                source.to_owned(),
                folder.to_owned(),
                name.to_owned(),
            ),
        }
    }
}

impl Execute for AssetFolderCommand {
    fn execute(&self, api: wikijs::Api) -> Result<(), Box<dyn Error>> {
        match self {
            AssetFolderCommand::List { parent_folder_id } => {
                asset_folder_list(api, *parent_folder_id)
            }
        }
    }
}

fn asset_list(api: wikijs::Api) -> Result<(), Box<dyn Error>> {
    let assets = api.asset_list(0, wikijs::asset::AssetKind::ALL)?;
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
    Ok(())
}

fn asset_folder_list(
    api: wikijs::Api,
    parent_folder_id: i64,
) -> Result<(), Box<dyn Error>> {
    let asset_folders = api.asset_folder_list(parent_folder_id)?;
    let mut builder = Builder::new();
    builder.push_record(["id", "slug", "name"]);
    for asset_folder in asset_folders {
        builder.push_record([
            asset_folder.id.to_string().as_str(),
            asset_folder.slug.as_str(),
            asset_folder.name.unwrap_or("".to_string()).as_str(),
        ]);
    }
    println!("{}", builder.build().with(Style::rounded()));
    Ok(())
}

fn asset_download(
    api: wikijs::Api,
    source: String,
    destination: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let asset = api.asset_download(source.clone())?;
    let mut file = File::create(destination.unwrap_or(source)).unwrap();
    file.write_all(&asset).unwrap();
    println!("{}: {}", "success".bold().green(), "asset downloaded");
    Ok(())
}

fn asset_upload(
    api: wikijs::Api,
    source: String,
    folder: i64,
    name: String,
) -> Result<(), Box<dyn Error>> {
    let data = std::fs::read(source)?;
    api.asset_upload(folder, name, data)?;
    println!("{}: {}", "success".bold().green(), "asset uploaded");
    Ok(())
}
