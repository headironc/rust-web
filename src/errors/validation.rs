use validator::{ValidationErrors, ValidationErrorsKind};

pub fn validation_error_handler(error: &ValidationErrors) -> String {
    let mut message = match_kind(error).join(", ");

    message += ".";

    message
}

fn match_kind(errors: &ValidationErrors) -> Vec<String> {
    let mut messages = Vec::new();
    let map = errors.to_owned().into_errors();

    for kind in map.values() {
        match kind {
            ValidationErrorsKind::Field(errors) => {
                let mut local = Vec::new();

                for error in errors.iter().cloned() {
                    let msg = error.message.unwrap().to_string();

                    local.push(msg);
                }

                messages.extend(local)
            }
            ValidationErrorsKind::Struct(errors) => {
                let errors = errors.as_ref();
                let local = match_kind(errors);
                messages.extend(local);
            }
            ValidationErrorsKind::List(errors_map) => {
                let errors = errors_map.values();
                let mut local = Vec::new();

                for error in errors {
                    let errors = error.as_ref();
                    local.extend(match_kind(errors));
                }

                messages.extend(local);
            }
        }
    }

    messages.sort();
    messages.dedup();

    messages
}
