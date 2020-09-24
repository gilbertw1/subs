use clap::ArgMatches;

pub fn load_config(matches: &ArgMatches) -> SubsConfig {
    SubsConfig {
        file: matches.value_of("file").unwrap().to_string(),
        output_file: matches.value_of("output-file").unwrap().to_string(),
        mappings: matches.value_of("mappings").map(|m| m.to_string()),
        mappings_file: matches.value_of("mappings-file").map(|m| m.to_string()),
        start_delim: matches.value_of("start-delim").unwrap().to_string(),
        end_delim: matches.value_of("end-delim").unwrap().to_string(),
    }
}

#[derive(Debug)]
pub struct SubsConfig {
    pub file: String,
    pub output_file: String,
    pub mappings: Option<String>,
    pub mappings_file: Option<String>,
    pub start_delim: String,
    pub end_delim: String,
}
