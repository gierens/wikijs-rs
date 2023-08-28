use wikijs::{Api, Credentials};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref API: Api = Api::new(
        "http://localhost".to_string(),
        Credentials::UsernamePassword(
            "admin@admin.com".to_string(),
            "password".to_string(),
            "local".to_string(),
        ),
    );
}
