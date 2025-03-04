use crate::state::State;

pub struct Yaml {
    indent: usize,
    text: String,
}

impl Yaml {
    pub fn new() -> Self {
        Yaml {
            text: "autoinstall:\n".to_string(),
            indent: 2,
        }
    }

    pub fn increase_indent(&mut self) {
        self.indent += 2;
    }

    pub fn decrease_indent(&mut self) {
        self.indent -= 2;
    }

    pub fn write_line(&mut self, str: String) {
        self.text += format!("{}{}\n", " ".repeat(self.indent), str).as_str();
    }

    pub fn write_vec(&mut self, label: String, elements: &Vec<String>) {
        if !elements.is_empty() {
            self.write_line(label);
            self.indent += 2;
            for element in elements {
                self.write_line(format!("- {}", element));
            }
            self.indent -= 2;
        }
    }
}

fn write_install_checkbox(yaml: &mut Yaml, label: &str, condition: bool) {
    if condition {
        yaml.write_line(label.to_string());
        yaml.increase_indent();
        yaml.write_line("install: true".to_string());
        yaml.decrease_indent();
    }
}

pub fn write_yaml(state: &State) -> String {
    let mut yaml = Yaml::new();
    yaml.write_line(format!("version: {}", &state.version));

    yaml.write_vec(
        "interactive-sections:".to_string(),
        &state.interactive_sections,
    );

    yaml.write_vec("early-commands:".to_string(), &state.early_commands);

    if !&state.locale.is_empty() {
        yaml.write_line(format!("locale: {}", &state.locale));
    }

    if state.refresh_installer.update {
        yaml.write_line("refresh-installer:".to_string());
        yaml.increase_indent();
        yaml.write_line("update: true".to_string());
        yaml.write_line(format!("channel: {}", &state.refresh_installer.channel));
        yaml.decrease_indent();
    }

    if !state.keyboard.layout.is_empty() {
        yaml.write_line("keyboard:".to_string());
        yaml.increase_indent();
        yaml.write_line(format!("layout: {}", &state.keyboard.layout));

        let variant_str = if state.keyboard.variant.is_empty() {
            "\"\"".to_string()
        } else {
            state.keyboard.variant.to_string()
        };
        yaml.write_line(format!("variant: {}", variant_str));

        let toggle_str = if state.keyboard.toggle.is_empty() {
            "null".to_string()
        } else {
            state.keyboard.toggle.to_string()
        };
        yaml.write_line(format!("toogle: {}", toggle_str));
        yaml.decrease_indent();
    }

    if state.source.search_drivers {
        yaml.write_line("source:".to_string());
        yaml.increase_indent();
        yaml.write_line("search_drivers: true".to_string());
        yaml.write_line(format!("id: {}", &state.source.id));
        yaml.decrease_indent();
    }

    if !&state.proxy.is_empty() {
        yaml.write_line(format!("proxy: {}", &state.proxy));
    }

    if !&state.identity.username.is_empty() {
        yaml.write_line("identity:".to_string());
        yaml.increase_indent();

        if !&state.identity.realname.is_empty() {
            yaml.write_line(format!("realname: '{}'", &state.identity.realname));
        }

        yaml.write_line(format!("username: {}", &state.identity.username));
        yaml.write_line(format!("password: '{}'", &state.identity.password_hash));
        yaml.write_line(format!("hostname: {}", &state.identity.hostname));
        yaml.decrease_indent();
    }

    if !&state.ubuntu_pro_token.is_empty() {
        yaml.write_line("ubuntu-pro:".to_string());
        yaml.increase_indent();
        yaml.write_line(format!("token: {}", &state.ubuntu_pro_token));
        yaml.decrease_indent();
    }

    yaml.write_line("ssh:".to_string());
    yaml.increase_indent();
    yaml.write_line(format!("install-server: {}", &state.ssh.install_server));
    if state.ssh.allow_pw {
        yaml.write_line("authorized-keys: []".to_string());
        yaml.write_line("allow-pw: true".to_string());
    } else {
        yaml.write_vec("authorized-keys".to_string(), &state.ssh.authorized_keys);
        yaml.write_line("allow-pw: false".to_string());
    }
    yaml.decrease_indent();

    write_install_checkbox(&mut yaml, "codecs:", state.codecs);
    write_install_checkbox(&mut yaml, "drivers:", state.drivers);
    write_install_checkbox(&mut yaml, "oem:", state.oem);

    yaml.write_vec("packages:".to_string(), &state.packages);

    yaml.text
}
