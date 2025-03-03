use crate::validators::*;

#[derive(Debug)]
pub struct State {
    pub version: i64,
    pub interactive_sections: Vec<String>,
    pub locale: String,
    pub keyboard: Keyboard,
    pub identity: Identity,
    pub codecs: bool,
    pub drivers: bool,
    pub oem: bool,
    pub snaps: Vec<String>,
    pub packages: Vec<String>,
    pub timezone: String,
    pub updates: String,

    pub errors: Vec<String>,
}

impl State {
    pub fn validate_fields(&mut self) {
        self.errors = Vec::new();
        match validate_locale(&self.locale) {
            Ok(_) => {}
            Err(e) => self.errors.push(e),
        }
    }
}

impl Default for State {
    fn default() -> State {
        State {
            version: 1,
            interactive_sections: Vec::new(),
            locale: String::from("en_US.UTF-8"),
            keyboard: Keyboard::default(),
            identity: Identity::default(),
            codecs: false,
            drivers: false,
            oem: true,
            snaps: Vec::new(),
            packages: Vec::new(),
            timezone: String::from("Europe/Amsterdam"),
            updates: String::new(),
            errors: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Keyboard {
    pub layout: String,
    pub variant: String,
    pub toggle: String,
}

impl Default for Keyboard {
    fn default() -> Keyboard {
        Keyboard {
            layout: "us".to_string(),
            variant: "".to_string(),
            toggle: "".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Identity {
    pub realname: String,
    pub username: String,
    pub password_hash: String,
    pub hostname: String,
}

impl Default for Identity {
    fn default() -> Identity {
        Identity {
            realname: String::new(),
            username: String::new(),
            password_hash: String::new(),
            hostname: String::new(),
        }
    }
}
