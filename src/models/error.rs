use std::collections::HashMap;

use serde::Serialize;
use validator::ValidationErrors;

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
    pub errors: Option<HashMap<String, String>>,
}

impl ErrorResponse {
    pub fn new(status: &str, message: &str, errors: Option<&ValidationErrors>) -> Self {
        let errors_map = match errors {
            Some(validation_errors) => {
                let mut map: HashMap<String, String> = HashMap::new();

                for (field, errors) in validation_errors.field_errors() {
                    let err_str = errors
                        .iter()
                        .map(|ve| match &ve.message {
                            Some(msg) => msg.clone().into_owned(),
                            None => ve.code.clone().into_owned(),
                        })
                        .collect::<Vec<String>>()
                        .join(",");

                    map.insert(field.into(), err_str);
                }

                Some(map)
            }
            None => None,
        };

        ErrorResponse {
            status: status.into(),
            message: message.into(),
            errors: errors_map,
        }
    }
}
