
mod cli;
mod conf;
mod mappings;
mod convert;

fn main() {

    let cli_app = cli::create_cli_app();
    let matches = cli_app.clone().get_matches();
    let config = conf::load_config(&matches);

    if config.mappings_file.is_some() && config.mappings_file.to_owned().unwrap() == config.file {
        panic!("MAPPINGS-FILE and FILE cannot reference the same file.");
    }

    let mappings = mappings::read_mappings(&config);
    convert::read_convert_write(&config, &mappings);
}
