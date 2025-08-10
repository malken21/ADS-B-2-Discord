use yaml_rust::{ YamlLoader, YamlEmitter };
use std::fs;

pub fn load() -> yaml_rust::Yaml {
    load_yaml("./config.yaml")
}

fn load_yaml(path: &str) -> yaml_rust::Yaml {
    let f = fs::read_to_string(path);
    let s = f.unwrap().to_string();
    return YamlLoader::load_from_str(&s).unwrap()[0].clone();
}
