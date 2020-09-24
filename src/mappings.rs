use crate::conf::SubsConfig;

use std::collections::HashMap;
use std::fs;
use std::path::Path;


pub fn read_mappings(conf: &SubsConfig) -> SubsMappings {
    let mappings_text =
        if conf.mappings_file.is_some() {
            let mappings_file = conf.mappings_file.to_owned().unwrap();
            if mappings_file == "-" {
                todo!("Read mappings from stdin.")
            } else {
                fs::read_to_string(Path::new(&mappings_file))
                    .expect("Couldn't read mappings file.")
            }
        } else {
            conf.mappings.to_owned().unwrap()
        };
    let mappings_map = parse_mappings(mappings_text);
    SubsMappings { map: mappings_map }
}

fn parse_mappings(mappings_text: String) -> HashMap<String,MappingItem> {
    let mut map = HashMap::new();
    for line in mappings_text.lines() {
        let trimmed_line = line.trim();
        if !trimmed_line.starts_with('#') && trimmed_line.contains('=') {
            let splitted: Vec<&str> = line.splitn(2, '=').collect();
            let sym = splitted[0].trim().to_string();
            let item_text = splitted[1].trim();
            let item =
                if item_text.starts_with('"') && item_text.ends_with('"') {
                    MappingItem::Value(item_text.strip_prefix('"').unwrap().strip_suffix('"').unwrap().to_string())
                } else {
                    MappingItem::Sym(item_text.to_string())
                };
            map.insert(sym, item);
        }
    }
    map
}

pub struct SubsMappings {
    map: HashMap<String,MappingItem>
}

impl SubsMappings {

    pub fn resolve_symbol(&self, sym: &str) -> Option<&str> {
        match self.map.get(sym) {
            Some(MappingItem::Value(value)) => Some(value),
            Some(MappingItem::Sym(sym)) => self.resolve_symbol(sym),
            None => None,
        }
    }
}

enum MappingItem {
    Value(String),
    Sym(String),
}
