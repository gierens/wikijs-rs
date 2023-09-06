use serde::Deserialize;

pub type Boolean = bool;
pub type Int = i64;
pub type Date = String;

#[derive(Deserialize, Debug)]
pub struct KeyValuePair {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct ResponseStatus {
    pub succeeded: Boolean,
    #[serde(rename = "errorCode")]
    pub error_code: Int,
    pub slug: String,
    pub message: Option<String>,
}

pub(crate) trait UnknownError {
    fn unknown_error_code(code: i64, message: String) -> Self;
    fn unknown_error_message(message: String) -> Self;
    fn unknown_error() -> Self;
}

pub(crate) trait KnownErrorCodes {
    fn known_error_codes() -> Vec<i64>;
    fn is_known_error_code(code: i64) -> bool;
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

pub(crate) fn classify_response_status_error<
    E: UnknownError + KnownErrorCodes + From<i64>,
>(
    response_status: ResponseStatus,
) -> E {
    if !E::is_known_error_code(response_status.error_code) {
        if let Some(message) = response_status.message {
            return E::unknown_error_code(response_status.error_code, message);
        }
    }
    response_status.error_code.into()
}
