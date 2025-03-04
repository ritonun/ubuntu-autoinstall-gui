use crate::utils::{self, Validators};

fn load_validators() -> Validators {
    let filepath = "validation.json";
    match utils::load_validators_from_file(filepath) {
        Ok(validators) => return validators,
        Err(e) => panic!("Failed to load validator file {} due to: {}", filepath, e),
    }
}

fn get_vec_into_string(vec: &Vec<String>) -> String {
    let mut string = String::new();
    for value in vec {
        string += value.as_str();
        string += ", ";
    }
    string
}

pub fn validate_locale(locale: &str) -> Result<(), String> {
    let validators = load_validators();

    if !validators.locale.contains(&locale.to_string()) {
        let possible_values = get_vec_into_string(&validators.locale);
        return Err(format!(
            "Locale value '{}' is not among the possible value, possible values are: {}",
            locale, possible_values
        ));
    }

    Ok(())
}

pub fn validate_interactive_sections(interactive_sections: &Vec<String>) -> Result<(), String> {
    let validators = load_validators();

    for section in interactive_sections {
        if !validators.sections.contains(section) {
            let possible_values = get_vec_into_string(&validators.sections);
            return Err(format!(
                "Interactive sections value '{}' is not among the possible value, which are {}",
                section, possible_values
            ));
        }
    }

    Ok(())
}
