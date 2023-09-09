use clap::Subcommand;
use colored::Colorize;
use tabled::{builder::Builder, settings::Style};
use crate::common::Execute;

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
}

impl Execute for UserCommand {
    fn execute(&self, api: wikijs::Api) {
        match self {
            UserCommand::Get { id } => user_get(api, *id),
            UserCommand::List { filter, order_by } => {
                user_list(api, filter.to_owned(), order_by.to_owned())
            }
        }
    }
}

fn user_get(api: wikijs::Api, id: i64) {
    match api.user_get(id) {
        Ok(user) => {
            let mut builder = Builder::new();
            builder.push_record(["key", "value"]);
            builder.push_record(["id", user.id.to_string().as_str()]);
            builder.push_record(["name", user.name.as_str()]);
            builder.push_record(["email", user.email.as_str()]);
            builder.push_record([
                "provider_key",
                user.provider_key.as_str(),
            ]);
            builder.push_record([
                "provider_name",
                user.provider_name.unwrap_or("".to_string()).as_str(),
            ]);
            builder.push_record([
                "provider_id",
                user.provider_id.unwrap_or("".to_string()).as_str(),
            ]);
            // providerIs2FACapable
            builder.push_record([
                "is_system",
                user.is_system.to_string().as_str(),
            ]);
            builder.push_record([
                "is_active",
                user.is_active.to_string().as_str(),
            ]);
            builder.push_record([
                "is_verified",
                user.is_verified.to_string().as_str(),
            ]);
            builder.push_record(["location", user.location.as_str()]);
            builder.push_record(["job_title", user.job_title.as_str()]);
            builder.push_record(["timezone", user.timezone.as_str()]);
            builder.push_record([
                "date_format",
                user.date_format.as_str(),
            ]);
            builder
                .push_record(["appearance", user.appearance.as_str()]);
            builder.push_record([
                "created_at",
                user.created_at.to_string().as_str(),
            ]);
            builder.push_record([
                "updated_at",
                user.updated_at.to_string().as_str(),
            ]);
            builder.push_record([
                "last_login_at",
                user.last_login_at.unwrap_or("".to_string()).as_str(),
            ]);
            // tfaIsActive
            // groups
            println!("{}", builder.build().with(Style::rounded()));
        }
        Err(e) => {
            eprintln!("{}: {}", "error".bold().red(), e.to_string());
            std::process::exit(1);
        }
    }
}

fn user_list(api: wikijs::Api, filter: Option<String>, order_by: Option<String>) {
    match api.user_list(filter, order_by) {
        Ok(users) => {
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
        }
        Err(e) => {
            eprintln!("{}: {}", "error".bold().red(), e.to_string());
            std::process::exit(1);
        }
    }
}
