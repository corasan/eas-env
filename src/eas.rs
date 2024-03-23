use std::{env, fs};
use std::io::{BufWriter, Write};
use serde_json::Value;
use log::{info, error};
use env_logger::Builder;
use log::LevelFilter;
use colored::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the EAS profile to use
    #[arg(short, long, default_value = "default")]
    profile: String,
}

pub fn create_env() {
  init_logger();
  let args = Args::parse();
  write_to_env(args.profile.to_string());
}

fn init_logger() {
  Builder::new()
		.format(|buf, record| {
			let level = record.level();
			writeln!(
				buf,
				"[{}] - {}",
				match level {
          log::Level::Error => level.to_string().red(),
          log::Level::Warn => level.to_string().yellow(),
          log::Level::Info => level.to_string().green(),
          log::Level::Debug => level.to_string().purple(),
          log::Level::Trace => level.to_string().cyan(),
				},
				record.args()
			)
		})
		.filter(None, LevelFilter::max())
		.init();
}

fn write_to_env(profile: String) {
  let eas = read_config();
  let p = profile.clone();
  let env = match eas["build"][profile]["env"].as_object() {
      Some(env) => env,
      None => {
          error!("'env' field not found");
          std::process::exit(1);
      }
  };
  if env.is_empty() {
      error!("'env' field has no keys");
      std::process::exit(1);
  }
  let dir = get_dir();
  let file = fs::File::create(format!("{}/.env.local", dir)).expect("Error creating .env file");
  
  let mut file = BufWriter::new(file);
  for (key, value) in env {
      write!(file, "{}={}\n", key, value).expect("Error writing to file");
  }
  info!("✅ Created .env.local with profile: {}\n", p);
}

fn read_config() -> Value {
  let dir = get_dir();
  let app_config = fs::read_to_string(format!("{}/app.json", dir)).expect("Error reading App config");
  let eas = fs::read_to_string(format!("{}/eas.json", dir)).expect("Error reading EAS config");
  let app_json: serde_json::Value = serde_json::from_str(&app_config).unwrap();
  let eas_json: serde_json::Value = serde_json::from_str(&eas).unwrap();
  let app_name = app_json["expo"]["name"].as_str().unwrap().replace("\"", "");
  info!("✅ EAS Configuration found for app: {}", app_name);
  return eas_json;
}

fn get_dir() -> String {
  let current_dir = env::current_dir().expect("Error getting current directory");
  let string = current_dir.to_str().expect("Error converting path to string");
  return string.to_string();
}
