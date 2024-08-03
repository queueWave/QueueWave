use clap::{Arg, Command};
use logging::{log_info};
pub fn init_cli() {
    log_info(&format!("Initializing Cli"));
    let matches = Command::new("SecureMQ CLI")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Manages SecureMQ")
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .required(false)
            .value_parser(clap::value_parser!(String)))
        .get_matches();

    if let Some(config) = matches.get_one::<String>("config") {
        log_info(&format!("Value for config: {}", config));
    } else {
        log_info(&format!("Using default config"));
    }
}
