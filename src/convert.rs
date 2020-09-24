use crate::mappings::SubsMappings;
use crate::conf::SubsConfig;

use std::io::{stdin, stdout, BufReader, BufWriter, BufRead, Write};
use std::fs::File;

pub fn read_convert_write(config: &SubsConfig, mappings: &SubsMappings) {
    let reader = get_reader(config);
    let mut writer = get_writer(config);

    for line in reader.lines() {
        let line = line.expect("Failed to read line from input file.");
        writer.write_all(convert_line(line, mappings, config).as_bytes()).expect("Error writing to output file.");
    }
}

fn get_writer(config: &SubsConfig) -> Box<dyn Write> {
    if config.output_file == "-" {
        Box::new(BufWriter::new(stdout()))
    } else {
        Box::new(BufWriter::new(File::create(&config.output_file).expect("Failed to open output file")))
    }
}

fn get_reader(config: &SubsConfig) -> Box<dyn BufRead> {
    if config.file == "-" {
        Box::new(BufReader::new(stdin()))
    } else {
        Box::new(BufReader::new(File::open(&config.file).expect("Failed to open input file")))
    }
}

fn convert_line(line: String, mappings: &SubsMappings, config: &SubsConfig) -> String {
    let mut splitted: Vec<&str> = line.splitn(2, &config.start_delim).collect();
    let mut result = String::new();
    while splitted.len() > 1 {
        result.push_str(splitted[0]);
        let inner_split: Vec<&str> = splitted[1].splitn(2, &config.end_delim).collect();
        if inner_split.len() > 1 {
            result.push_str(mappings.resolve_symbol(inner_split[0].trim()).unwrap_or(""));
            splitted = inner_split[1].splitn(2, &config.start_delim).collect();
        } else {
           result.push_str(inner_split[0]);
           splitted = Vec::new();
        }
    }

    if !splitted.is_empty() {
        result.push_str(splitted[0]);
    }

    result.push('\n');
    result
}
