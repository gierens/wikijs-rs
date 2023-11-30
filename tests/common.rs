use lazy_static::lazy_static;
use wikijs::{Api, Credentials};

lazy_static! {
    pub static ref API: Api = Api::new(
        "http://localhost".to_string(),
        Credentials::UsernamePassword(
            "admin@admin.com".to_string(),
            "password".to_string(),
            "local".to_string(),
        ),
    )
    .unwrap_or_else(|e| panic!("Error creating API: {}", e));
}
