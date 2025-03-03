#[derive(Debug)]
pub struct State {
    pub version: i64,
    pub interactive_sections: Vec<String>,
    pub locale: String,
    pub keyboards: Vec<Keyboard>,
    pub identity: Identity,
    pub codecs: bool,
    pub drivers: bool,
    pub oem: bool,
    pub snaps: Vec<String>,
    pub packages: Vec<String>,
    pub timezone: String,
    pub updates: String,
}

impl Default for State {
    fn default() -> State {
        State {
            version: 1,
            interactive_sections: Vec::new(),
            locale: String::from("en_US.UTF-8"),
            keyboards: Vec::new(),
            identity: Identity::default(),
            codecs: false,
            drivers: false,
            oem: true,
            snaps: Vec::new(),
            packages: Vec::new(),
            timezone: String::from("Europe/Amsterdam"),
            updates: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct Keyboard {
    pub keyboard_layout: String,
    pub keyboard_variant: String,
    pub keyboard_toggle: bool,
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
