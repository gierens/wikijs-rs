pub(crate) trait UnknownError {
    fn unknown_error_code(code: i64, message: String) -> Self;
    fn unknown_error_message(message: String) -> Self;
    fn unknown_error() -> Self;
}

pub(crate) fn classify_response_error<E: UnknownError + From<i64>>(
    response_errors: Option<Vec<graphql_client::Error>>,
) -> E {
    if response_errors.is_some() {
        let errors = response_errors.unwrap();
        if errors.len() > 0 {
            let error = errors[0].clone();
            if error.extensions.is_some() {
                let extensions = error.extensions.unwrap();
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
