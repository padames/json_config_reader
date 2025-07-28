use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use clap::{Arg, Command};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    settings: Vec<HashMap<String, i32>>,
}

impl Default for Config {
    fn default() -> Self {
        let mut settings = Vec::new();
        
        let mut setting_a = HashMap::new();
        setting_a.insert("SETTING A".to_string(), 34);
        settings.push(setting_a);
        
        let mut setting_b = HashMap::new();
        setting_b.insert("SETTING B".to_string(), 20);
        settings.push(setting_b);
        
        let mut setting_c = HashMap::new();
        setting_c.insert("SETTING C".to_string(), 100);
        settings.push(setting_c);
        
        Config { settings }
    }
}

fn main() {
    let matches = Command::new("json_config_reader")
        .version("0.1.0")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Configuration file to read"),
        )
        .get_matches();

    let config = if let Some(filename) = matches.get_one::<String>("file") {
        match read_config_from_file(filename) {
            Ok(config) => {
                println!("Successfully loaded configuration from: {}", filename);
                config
            },
            Err(e) => {
                eprintln!("Error reading config file '{}': {}", filename, e);
                eprintln!("Using default configuration instead.");
                Config::default()
            }
        }
    } else {
        println!("No config file specified, using default configuration.");
        Config::default()
    };
    
    // Print configuration as KEY=VALUE pairs
    for setting_map in &config.settings {
        for (key, value) in setting_map {
            println!("{}={}", key, value);
        }
    }
}

fn read_config_from_file(filename: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(filename)?;
    let config: Config = serde_json::from_str(&contents)?;
    Ok(config)
}
