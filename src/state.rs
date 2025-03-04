use crate::validators::*;

#[derive(Debug)]
pub struct State {
    pub version: i64,
    pub interactive_sections: Vec<String>,
    pub early_commands: Vec<String>,
    pub locale: String,
    pub refresh_installer: RefreshInstaller,
    pub keyboard: Keyboard,
    pub source: Source,
    pub proxy: String,
    pub identity: Identity,
    pub ubuntu_pro_token: String,
    pub ssh: Ssh,
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
        match validate_interactive_sections(&self.interactive_sections) {
            Ok(_) => {}
            Err(e) => self.errors.push(e),
        }
    }
}

impl Default for State {
    fn default() -> State {
        State {
            version: 1,
            interactive_sections: vec!["*".to_string()],
            early_commands: Vec::new(),
            locale: String::from("en_US.UTF-8"),
            refresh_installer: RefreshInstaller::default(),
            keyboard: Keyboard::default(),
            source: Source::default(),
            proxy: String::new(),
            identity: Identity::default(),
            ubuntu_pro_token: String::new(),
            ssh: Ssh::default(),
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
pub struct Ssh {
    pub install_server: bool,
    pub authorized_keys: Vec<String>,
    pub allow_pw: bool,
}

impl Default for Ssh {
    fn default() -> Self {
        Ssh {
            install_server: false,
            authorized_keys: Vec::new(),
            allow_pw: true,
        }
    }
}

#[derive(Debug)]
pub struct Source {
    pub search_drivers: bool,
    pub id: String,
}

impl Default for Source {
    fn default() -> Self {
        Source {
            search_drivers: true,
            id: "ubuntu-desktop-minimal".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct RefreshInstaller {
    pub update: bool,
    pub channel: String,
}

impl Default for RefreshInstaller {
    fn default() -> Self {
        RefreshInstaller {
            update: false,
            channel: "stable/ubuntu-$REL".to_string(),
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
