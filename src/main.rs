use clap::{App, Arg};
use std::io::prelude::*;

const DEFAULT_CONFIG: &str = "/etc/thonkfan.toml";

#[derive(Debug, serde::Deserialize)]
struct Curve {
    level: usize,
    low: u16,
    high: u16,
}

#[derive(Debug, serde::Deserialize)]
struct Config {
    fan: String,
    thermal: String,
    curve: Vec<Curve>,
}

fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut fan_handle = std::fs::OpenOptions::new().write(true).open(&config.fan)?;
    let mut temp_handle = std::fs::File::open(&config.thermal)?;

    let mut write_level = |n: usize| fan_handle.write_all(format!("level {}", n).as_bytes());

    let mut read_temp = || -> Result<u16, Box<dyn std::error::Error>> {
        let mut contents = String::new();
        temp_handle.seek(std::io::SeekFrom::Start(0))?;
        temp_handle.read_to_string(&mut contents)?;
        let n = contents.trim().parse::<usize>()? / 1000;
        Ok(n as u16)
    };

    let mut level_index = 0;
    let mut level = &config.curve[level_index];

    write_level(config.curve[level_index].level)?;

    loop {
        let temp = read_temp()?;

        if let Some(l) = if temp > level.high {
            level_index += 1;
            Some(&config.curve[level_index])
        } else if temp < level.low {
            level_index -= 1;
            Some(&config.curve[level_index])
        } else {
            None
        } {
            level = l;
            write_level(level.level)?;
        }

        println!("{}C {} {}-{}", temp, level.level, level.low, level.high,);

        std::thread::sleep(std::time::Duration::from_millis(5000));
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("thonkfan")
        .arg(
            Arg::with_name("config")
                .short("c")
                .takes_value(true)
                .help("Path to config file"),
        )
        .get_matches();

    println!("thonkfan beta");

    let config_path = matches.value_of("config").unwrap_or(DEFAULT_CONFIG);

    println!("reading config from {}", config_path);

    let config_src = std::fs::read_to_string(config_path).expect("config not found");
    let config: Config = toml::from_str(&config_src)?;

    run(&config)?;

    Ok(())
}
