use crate::utils::{self, Validators};

fn load_validators() -> Validators {
    let filepath = "validation.json";
    match utils::load_validators_from_file(filepath) {
        Ok(validators) => return validators,
        Err(e) => panic!("Failed to load validator file {} due to: {}", filepath, e),
    }
}

pub fn validate_locale(locale: &str) -> Result<(), String> {
    let validators = load_validators();

    if !validators.locale.contains(&locale.to_string()) {
        let mut possible_values = String::new();
        for ok_value in validators.locale {
            possible_values += ok_value.as_str();
            possible_values += ", ";
        }
        return Err(format!(
            "Locale value is not among the possible value, possible values are: {}",
            possible_values
        ));
    }

    Ok(())
}
