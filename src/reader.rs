use std::io::Read;
use yaml_rust::YamlLoader;

use crate::state::State;

pub fn read_file(path: &str) -> String {
    let mut file =
        std::fs::File::open(path).expect(format!("Failed to open file {}", path).as_str());
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect(format!("Failed to read file {}", path).as_str());
    contents
}

pub fn parse_yaml_to_state(contents: String, state: &mut State) {
    let yaml_content = YamlLoader::load_from_str(contents.as_str()).expect("Failed to parse YAML");
    let yaml = &yaml_content[0];

    // parse
    if let Some(version) = yaml["autoinstall"]["version"].as_i64() {
        state.version = version;
        println!("Version: {} ", version);
    }

    if let Some(_) = yaml["autoinstall"]["refresh-installer"].as_hash() {
        if let Some(update) = yaml["autoinstall"]["refresh-installer"]["update"].as_bool() {
            state.refresh_installer.update = update;
            println!("refresh_installer: {} ", update);
        }
        if let Some(channel) = yaml["autoinstall"]["refresh-installer"]["channel"].as_str() {
            state.refresh_installer.channel = channel.to_string();
            println!("refresh_installer: {} ", channel);
        }
    }
}
