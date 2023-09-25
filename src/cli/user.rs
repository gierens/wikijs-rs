use crate::common::Execute;
use clap::Subcommand;
use colored::Colorize;
use std::error::Error;
use tabled::{builder::Builder, settings::Style};

#[derive(Subcommand)]
pub(crate) enum UserCommand {
    #[clap(about = "Get a user")]
    Get {
        #[clap(help = "User ID")]
        id: i64,
    },

    #[clap(about = "List users")]
    List {
        #[clap(short, long, help = "Filter users by this")]
        filter: Option<String>,

        #[clap(short, long, help = "Order users by this")]
        order_by: Option<String>,
    },

    #[clap(about = "Create a user")]
    Create {
        #[clap(help = "Email address")]
        email: String,

        #[clap(help = "Name")]
        name: String,

        #[clap(
            short,
            long,
            help = "Password, required for local provider, \
            and length might matter."
        )]
        password: Option<String>,

        #[clap(
            short = 'P',
            long,
            help = "Provider key",
            default_value = "local"
        )]
        provider_key: String,

        #[clap(short, long, help = "Groups")]
        groups: Vec<i64>,

        #[clap(short, long, help = "Must change password")]
        must_change_password: Option<bool>,

        #[clap(short, long, help = "Send welcome email")]
        send_welcome_email: Option<bool>,
    },

    #[clap(about = "Activate a user")]
    Activate {
        #[clap(help = "User ID")]
        id: i64,
    },

    #[clap(about = "Deactivate a user")]
    Deactivate {
        #[clap(help = "User ID")]
        id: i64,
    },
}

impl Execute for UserCommand {
    fn execute(&self, api: wikijs::Api) -> Result<(), Box<dyn Error>> {
        match self {
            UserCommand::Get { id } => user_get(api, *id),
            UserCommand::List { filter, order_by } => {
                user_list(api, filter.to_owned(), order_by.to_owned())
            }
            UserCommand::Create {
                email,
                name,
                password,
                provider_key,
                groups,
                must_change_password,
                send_welcome_email,
            } => user_create(
                api,
                email.to_owned(),
                name.to_owned(),
                password.to_owned(),
                provider_key.to_owned(),
                groups.to_owned(),
                *must_change_password,
                *send_welcome_email,
            ),
            UserCommand::Activate { id } => user_activate(api, *id),
            UserCommand::Deactivate { id } => user_deactivate(api, *id),
        }
    }
}

fn user_get(api: wikijs::Api, id: i64) -> Result<(), Box<dyn Error>> {
    let user = api.user_get(id)?;
    let mut builder = Builder::new();
    builder.push_record(["key", "value"]);
    builder.push_record(["id", user.id.to_string().as_str()]);
    builder.push_record(["name", user.name.as_str()]);
    builder.push_record(["email", user.email.as_str()]);
    builder.push_record(["provider_key", user.provider_key.as_str()]);
    builder.push_record([
        "provider_name",
        user.provider_name.unwrap_or("".to_string()).as_str(),
    ]);
    builder.push_record([
        "provider_id",
        user.provider_id.unwrap_or("".to_string()).as_str(),
    ]);
    // providerIs2FACapable
    builder.push_record(["is_system", user.is_system.to_string().as_str()]);
    builder.push_record(["is_active", user.is_active.to_string().as_str()]);
    builder.push_record(["is_verified", user.is_verified.to_string().as_str()]);
    builder.push_record(["location", user.location.as_str()]);
    builder.push_record(["job_title", user.job_title.as_str()]);
    builder.push_record(["timezone", user.timezone.as_str()]);
    builder.push_record(["date_format", user.date_format.as_str()]);
    builder.push_record(["appearance", user.appearance.as_str()]);
    builder.push_record(["created_at", user.created_at.to_string().as_str()]);
    builder.push_record(["updated_at", user.updated_at.to_string().as_str()]);
    builder.push_record([
        "last_login_at",
        user.last_login_at.unwrap_or("".to_string()).as_str(),
    ]);
    // tfaIsActive
    // groups
    println!("{}", builder.build().with(Style::rounded()));
    Ok(())
}

fn user_list(
    api: wikijs::Api,
    filter: Option<String>,
    order_by: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let users = api.user_list(filter, order_by)?;
    let mut builder = Builder::new();
    builder.push_record([
        "id",
        "name",
        "email",
        "provider_key",
        "is_system",
        "is_active",
        "created_at",
        "last_login_at",
    ]);
    for user in users {
        builder.push_record([
            user.id.to_string().as_str(),
            user.name.as_str(),
            user.email.as_str(),
            user.provider_key.as_str(),
            user.is_system.to_string().as_str(),
            user.is_active.to_string().as_str(),
            user.created_at.to_string().as_str(),
            user.last_login_at.unwrap_or("".to_string()).as_str(),
        ]);
    }
    println!("{}", builder.build().with(Style::rounded()));
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn user_create(
    api: wikijs::Api,
    email: String,
    name: String,
    password: Option<String>,
    provider_key: String,
    groups: Vec<i64>,
    must_change_password: Option<bool>,
    send_welcome_email: Option<bool>,
) -> Result<(), Box<dyn Error>> {
    api.user_create(
        email,
        name,
        password,
        provider_key,
        groups.iter().map(|x| Some(*x)).collect(),
        must_change_password,
        send_welcome_email,
    )?;
    println!("{}: User created", "success".bold().green());
    Ok(())
}

fn user_activate(api: wikijs::Api, id: i64) -> Result<(), Box<dyn Error>> {
    api.user_activate(id)?;
    println!("{}: User activated", "success".bold().green());
    Ok(())
}

fn user_deactivate(api: wikijs::Api, id: i64) -> Result<(), Box<dyn Error>> {
    api.user_deactivate(id)?;
    println!("{}: User deactivated", "success".bold().green());
    Ok(())
}
