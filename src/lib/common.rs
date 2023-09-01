use serde::Deserialize;

pub type Boolean = bool;
pub type Int = i64;
pub type Date = String;

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct ResponseStatus {
    succeeded: Boolean,
    #[serde(rename = "errorCode")]
    error_code: Int,
    slug: String,
    message: Option<String>,
}

pub(crate) trait UnknownError {
    fn unknown_error_code(code: i64, message: String) -> Self;
    fn unknown_error_message(message: String) -> Self;
    fn unknown_error() -> Self;
}

pub(crate) fn classify_response_error<E: UnknownError + From<i64>>(
    response_errors: Option<Vec<graphql_client::Error>>,
) -> E {
    if let Some(errors) = response_errors {
        if !errors.is_empty() {
            let error = errors[0].clone();
            if let Some(extensions) = error.extensions {
                if extensions.contains_key("exception") {
                    let exception = extensions.get("exception").unwrap();
                    if exception.get("code").is_some() {
                        let code = exception.get("code").unwrap();
                        return code.as_i64().unwrap().into();
                    }
                }
            }
            return E::unknown_error_message(error.message);
        }
    }
    E::unknown_error()
}
