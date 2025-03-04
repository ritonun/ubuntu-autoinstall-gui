use crate::state::RefreshInstaller;
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

pub fn validate_source_id(source_id: &str) -> Result<(), String> {
    let validators = load_validators();

    if !validators.source_id.contains(&source_id.to_string()) {
        let possible_values = get_vec_into_string(&validators.source_id);
        return Err(format!(
            "Source ID value '{}' is not among the possible value, which are: {}",
            source_id, possible_values
        ));
    }

    Ok(())
}

pub fn validate_locale(locale: &str) -> Result<(), String> {
    let validators = load_validators();

    if !validators.locale.contains(&locale.to_string()) {
        let possible_values = get_vec_into_string(&validators.locale);
        return Err(format!(
            "Locale value '{}' is not among the possible value, which are: {}",
            locale, possible_values
        ));
    }

    Ok(())
}

pub fn validate_keyboard_layout(keyboard_layout: &str) -> Result<(), String> {
    let validators = load_validators();

    if !validators
        .keyboard_layout
        .contains(&keyboard_layout.to_string())
    {
        let possible_values = get_vec_into_string(&validators.keyboard_layout);
        return Err(format!(
            "Keyboard layout value '{}' is not among the possible value, which are: {}",
            keyboard_layout, possible_values
        ));
    }

    Ok(())
}

pub fn validate_keyboard_toggle(keyboard_toggle: &str) -> Result<(), String> {
    if keyboard_toggle.is_empty() {
        return Ok(());
    }
    let validators = load_validators();

    if !validators
        .keyboard_toggle
        .contains(&keyboard_toggle.to_string())
    {
        let possible_values = get_vec_into_string(&validators.keyboard_toggle);
        return Err(format!(
            "Keyboard toggle value '{}' is not among the possible value, which are: {}",
            keyboard_toggle, possible_values
        ));
    }

    Ok(())
}

pub fn validate_keyboard_variant(keyboard_variant: &str) -> Result<(), String> {
    if keyboard_variant.is_empty() {
        return Ok(());
    }
    let validators = load_validators();

    if !validators
        .keyboard_variant
        .contains(&keyboard_variant.to_string())
    {
        let possible_values = get_vec_into_string(&validators.keyboard_variant);
        return Err(format!(
            "Keyboard variant value '{}' is not among the possible value, which are: {}",
            keyboard_variant, possible_values
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

pub fn validate_refresh_install(refresh_installer: &RefreshInstaller) -> Result<(), String> {
    if refresh_installer.update {
        if refresh_installer.channel.is_empty() {
            return Err(
                "Refresh Installer Channel field cannot be empty if update is checked".to_string(),
            );
        }
    }
    Ok(())
}

pub fn validate_passwd(passwd: &String) -> Result<(), String> {
    if passwd.len() != 106 {
        return Err(
            "Password is not valid. Please check documentation or use mkpasswd --method=SHA-512 to convert your password".to_string(),
        );
    }

    Ok(())
}
