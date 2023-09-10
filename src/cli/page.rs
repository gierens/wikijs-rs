use crate::common::Execute;
use clap::Subcommand;
use colored::Colorize;
use std::error::Error;
use std::io::Error as IoError;
use std::io::Write;
use tabled::{builder::Builder, settings::Style};
use tempfile::Builder as TempFileBuilder;

#[derive(Subcommand)]
pub(crate) enum PageCommand {
    #[clap(about = "Get a page")]
    Get {
        #[clap(help = "Page ID")]
        id: i64,
    },

    #[clap(about = "List pages")]
    List {},

    #[clap(about = "Delete a page")]
    Delete {
        #[clap(help = "Page ID")]
        id: i64,
    },

    #[clap(about = "Render a page")]
    Render {
        #[clap(help = "Page ID")]
        id: i64,
    },

    #[clap(about = "Create a page")]
    Create {
        #[clap(short, long, help = "Page content", default_value = "...")]
        content: String,

        #[clap(short, long, help = "Page description", default_value = "")]
        description: String,

        #[clap(short, long, help = "Page editor", default_value = "markdown")]
        editor: String,

        #[clap(
            short = 'p',
            long,
            help = "Page is private",
            default_value = "false"
        )]
        is_private: bool,

        #[clap(
            short = 'P',
            long,
            help = "Page is published",
            default_value = "true"
        )]
        is_published: bool,

        #[clap(short, long, help = "Page locale", default_value = "en")]
        locale: String,

        #[clap(help = "Page path")]
        path: String,

        // #[clap(help = "Page publish start date")]
        // publish_start_date: Option<String>,

        // #[clap(help = "Page publish end date")]
        // publish_end_date: Option<String>,

        // #[clap(help = "Page CSS script")]
        // script_css: Option<String>,

        // #[clap(help = "Page JS script")]
        // script_js: Option<String>,
        #[clap(short = 'T', long, help = "Page tags")]
        tags: Vec<String>,

        #[clap(short, long, help = "Page title")]
        title: Option<String>,
    },

    #[clap(about = "Update a page")]
    Update {
        #[clap(help = "Page ID")]
        id: i64,

        #[clap(short, long, help = "Page content")]
        content: Option<String>,

        #[clap(short, long, help = "Page description")]
        description: Option<String>,

        #[clap(short, long, help = "Page editor")]
        editor: Option<String>,

        #[clap(short = 'P', long, help = "Page is private")]
        is_private: Option<bool>,

        #[clap(short = 'b', long, help = "Page is published")]
        is_published: Option<bool>,

        #[clap(short, long, help = "Page locale")]
        locale: Option<String>,

        #[clap(short, long, help = "Page path")]
        path: Option<String>,

        // #[clap(help = "Page publish start date")]
        // publish_start_date: Option<String>,

        // #[clap(help = "Page publish end date")]
        // publish_end_date: Option<String>,

        // #[clap(help = "Page CSS script")]
        // script_css: Option<String>,

        // #[clap(help = "Page JS script")]
        // script_js: Option<String>,
        #[clap(short = 'T', long, help = "Page tags")]
        tags: Option<Vec<String>>,

        #[clap(
            short,
            long,
            help = "Remove tags",
            action,
            conflicts_with = "tags"
        )]
        no_tags: bool,

        #[clap(short, long, help = "Page title")]
        title: Option<String>,
    },

    #[clap(about = "Update a page's content")]
    UpdateContent {
        #[clap(help = "Page ID")]
        id: i64,

        #[clap(help = "Page content")]
        content: String,
    },

    #[clap(about = "Edit a page")]
    Edit {
        #[clap(help = "Page ID")]
        id: i64,

        #[clap(
            short,
            long,
            help = "Editor to use for editing pages",
            default_value = "vi",
            env = "EDITOR"
        )]
        editor: String,
    },
}

impl Execute for PageCommand {
    fn execute(&self, api: wikijs::Api) -> Result<(), Box<dyn Error>> {
        match self {
            PageCommand::Get { id } => page_get(api, *id),
            PageCommand::List {} => page_list(api),
            PageCommand::Delete { id } => page_delete(api, *id),
            PageCommand::Render { id } => page_render(api, *id),
            PageCommand::Create {
                content,
                description,
                editor,
                is_private,
                is_published,
                locale,
                path,
                // publish_start_date,
                // publish_end_date,
                // script_css,
                // script_js,
                tags,
                title,
            } => page_create(
                api,
                content.to_string(),
                description.to_string(),
                editor.to_string(),
                *is_private,
                *is_published,
                locale.to_string(),
                path.to_string(),
                // publish_start_date.to_string(),
                // publish_end_date.to_string(),
                // script_css.to_string(),
                // script_js.to_string(),
                tags.to_vec(),
                title.clone(),
            ),
            PageCommand::Update {
                id,
                content,
                description,
                editor,
                is_private,
                is_published,
                locale,
                path,
                // publish_start_date,
                // publish_end_date,
                // script_css,
                // script_js,
                tags,
                no_tags,
                title,
            } => page_update(
                api,
                *id,
                content.to_owned(),
                description.to_owned(),
                editor.to_owned(),
                *is_private,
                *is_published,
                locale.to_owned(),
                path.to_owned(),
                // publish_start_date.to_owned(),
                // publish_end_date.to_owned(),
                // script_css.to_owned(),
                // script_js.to_owned(),
                tags.to_owned(),
                *no_tags,
                title.to_owned(),
            ),
            PageCommand::UpdateContent { id, content } => {
                page_update_content(api, *id, content.to_string())
            }
            PageCommand::Edit { id, editor } => {
                page_edit(api, *id, editor.to_string())
            }
        }
    }
}

fn page_get(api: wikijs::Api, id: i64) -> Result<(), Box<dyn Error>> {
    let page = api.page_get(id)?;
    let mut builder = Builder::new();
    builder.push_record(["key", "value"]);
    builder.push_record(["id", page.id.to_string().as_str()]);
    builder.push_record(["path", page.path.to_string().as_str()]);
    builder.push_record(["hash", page.hash.to_string().as_str()]);
    builder.push_record(["title", page.title.as_str()]);
    // TODO description
    builder.push_record(["is_private", page.is_private.to_string().as_str()]);
    builder
        .push_record(["is_published", page.is_published.to_string().as_str()]);
    builder.push_record([
        "private_ns",
        page.private_ns.unwrap_or("".to_string()).as_str(),
    ]);
    builder.push_record([
        "publish_start_date",
        &page.publish_start_date.to_string(),
    ]);
    builder
        .push_record(["publish_end_date", &page.publish_end_date.to_string()]);
    // TODO tags
    // TODO content
    // TODO toc
    // TODO render
    builder.push_record(["content_type", page.content_type.as_str()]);
    builder.push_record(["created_at", &page.created_at.to_string()]);
    builder.push_record(["updated_at", &page.updated_at.to_string()]);
    builder.push_record(["editor", page.editor.as_str()]);
    builder.push_record(["locale", page.locale.as_str()]);
    // TODO script_css
    // TODO script_js
    builder.push_record(["author_id", page.author_id.to_string().as_str()]);
    builder.push_record(["author_name", page.author_name.as_str()]);
    builder.push_record(["author_email", page.author_email.as_str()]);
    builder.push_record(["creator_id", page.creator_id.to_string().as_str()]);
    builder.push_record(["creator_name", page.creator_name.as_str()]);
    builder.push_record(["creator_email", page.creator_email.as_str()]);
    println!("{}", builder.build().with(Style::rounded()));
    Ok(())
}

fn page_list(api: wikijs::Api) -> Result<(), Box<dyn Error>> {
    let pages = api.page_list()?;
    let mut builder = Builder::new();
    builder.push_record([
        "id",
        "locate",
        "path",
        "title",
        "content_type",
        "is_published",
        "is_private",
        "private_ns",
        "created_at",
        "updated_at",
    ]);
    for page in pages {
        builder.push_record([
            page.id.to_string().as_str(),
            page.path.as_str(),
            page.locale.as_str(),
            page.title.unwrap_or("".to_string()).as_str(),
            // TODO description
            page.content_type.as_str(),
            page.is_published.to_string().as_str(),
            page.is_private.to_string().as_str(),
            page.private_ns.unwrap_or("".to_string()).as_str(),
            page.created_at.to_string().as_str(),
            page.updated_at.to_string().as_str(),
            // TODO tags
        ]);
    }
    println!("{}", builder.build().with(Style::rounded()));
    Ok(())
}

fn page_delete(api: wikijs::Api, id: i64) -> Result<(), Box<dyn Error>> {
    api.page_delete(id)?;
    println!("{}: Page deleted", "success".bold().green());
    Ok(())
}

fn page_render(api: wikijs::Api, id: i64) -> Result<(), Box<dyn Error>> {
    api.page_render(id)?;
    println!("{}: Page rendered", "success".bold().green());
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn page_create(
    api: wikijs::Api,
    content: String,
    description: String,
    editor: String,
    is_private: bool,
    is_published: bool,
    locale: String,
    path: String,
    // publish_start_date: String,
    // publish_end_date: String,
    // script_css: String,
    // script_js: String,
    tags: Vec<String>,
    title: Option<String>,
) -> Result<(), Box<dyn Error>> {
    api.page_create(
        content,
        description,
        editor,
        is_published,
        is_private,
        locale,
        path.clone(),
        None,
        None,
        None,
        None,
        tags.iter().map(|s| Some(s.clone())).collect(),
        title.unwrap_or(path.split('/').last().unwrap().to_string()),
    )?;
    println!("{}: Page created", "success".bold().green());
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn page_update(
    api: wikijs::Api,
    id: i64,
    content: Option<String>,
    description: Option<String>,
    editor: Option<String>,
    is_private: Option<bool>,
    is_published: Option<bool>,
    locale: Option<String>,
    path: Option<String>,
    // publish_start_date: Option<String>,
    // publish_end_date: Option<String>,
    // script_css: Option<String>,
    // script_js: Option<String>,
    tags: Option<Vec<String>>,
    no_tags: bool,
    title: Option<String>,
) -> Result<(), Box<dyn Error>> {
    api.page_update(
        id,
        content,
        description,
        editor,
        is_published,
        is_private,
        locale,
        path,
        None,
        None,
        None,
        None,
        if no_tags {
            Some(Vec::new())
        } else {
            tags.map(|tags| {
                tags.iter()
                    .map(|s| Some(s.clone()))
                    .collect::<Vec<Option<String>>>()
            })
        },
        title,
    )?;
    println!("{}: Page updated", "success".bold().green());
    Ok(())
}

fn page_update_content(
    api: wikijs::Api,
    id: i64,
    content: String,
) -> Result<(), Box<dyn Error>> {
    api.page_update_content(id, content)?;
    println!("{}: Page content updated", "success".bold().green());
    Ok(())
}

fn page_edit(
    api: wikijs::Api,
    id: i64,
    editor: String,
) -> Result<(), Box<dyn Error>> {
    let page = api.page_get(id)?;
    let file = match page.editor.as_str() {
        "markdown" => TempFileBuilder::new().suffix(".md").tempfile(),
        _ => TempFileBuilder::new().tempfile(),
    }?;
    file.reopen()?.write_all(page.content.as_bytes())?;
    let mut child = std::process::Command::new(editor)
        .arg(file.path())
        .spawn()?;
    let status = child.wait()?;
    if !status.success() {
        return Err(Box::new(IoError::new(
            std::io::ErrorKind::Other,
            "Editor exited with non-zero status code",
        )));
    }
    let content = std::fs::read_to_string(file.path())?;
    api.page_update_content(id, content)?;
    // TODO a generic success print function could be useful
    println!("{}: Page content updated", "success".bold().green());
    Ok(())
}
