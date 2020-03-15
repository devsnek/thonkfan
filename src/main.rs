use clap::{App, Arg};
use std::io::prelude::*;

const DEFAULT_CONFIG: &str = "/etc/thonkfan.toml";
const HWMON_ROOT: &str = "/sys/devices/platform/thinkpad_hwmon/hwmon/";
const HWMON_DEVICE: &str = "temp1_input";

#[derive(Debug, serde::Deserialize)]
struct Curve {
    level: usize,
    low: u16,
    high: u16,
}

#[derive(Debug, serde::Deserialize)]
struct Config {
    fan: String,
    curve: Vec<Curve>,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn run(config: &Config) -> Result<()> {
    let mut fan_handle = std::fs::OpenOptions::new()
        .write(true)
        .open(&config.fan)
        .expect("unable to open fan handle");
    let temp_path = std::fs::read_dir(HWMON_ROOT)?
        .nth(0)
        .unwrap()?
        .path()
        .join(HWMON_DEVICE);
    println!("opening hwmon {:?}", temp_path);
    let mut temp_handle = std::fs::File::open(temp_path).expect("unable to open temp handle");

    let mut write_level = |n: usize| fan_handle.write_all(format!("level {}", n).as_bytes());

    let mut read_temp = || -> Result<u16> {
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

        if if temp > level.high {
            level_index = level_index.saturating_add(1);
            true
        } else if temp < level.low {
            level_index = level_index.saturating_sub(1);
            true
        } else {
            false
        } {
            level = &config.curve[level_index];
            write_level(level.level)?;
        }

        println!("{}C {} {}-{}", temp, level.level, level.low, level.high,);

        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}

fn main() -> Result<()> {
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
