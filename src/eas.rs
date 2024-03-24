use clap::Parser;
use colored::*;
use env_logger::Builder;
use log::LevelFilter;
use log::{error, info};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::{env, fs};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// Name of the EAS profile to use
  #[arg(short, long, default_value = "default")]
  profile: String,

  /// Path to the app directory containing the app.json and eas.json files
  #[arg(long)]
  app_dir: Option<PathBuf>,
}

pub fn create_env() {
  init_logger();
  let args = Args::parse();
  let profile = args.profile;
  let mut dir = args.app_dir;
  if dir.is_none() {
    dir = Some(env::current_dir().expect("Failed to get current directory"));
  }
  write_to_env(profile, dir.unwrap().to_str().unwrap().to_string());
}

fn write_to_env(profile: String, path: String) {
  let p = profile.clone();
  let eas_env = get_eas_env(profile, path.clone());
  let file =
    fs::File::create(format!("{}/.env.local", path.clone())).expect("Error creating .env file");

  let mut file = BufWriter::new(file);
  for (key, value) in eas_env {
    write!(file, "{}={}\n", key, value).expect("Error writing to file");
  }
  info!("✅ Created .env.local with profile: {}\n", p);
}

fn get_eas_env(profile: String, path: String) -> serde_json::Map<String, serde_json::Value> {
  let app_config =
    fs::read_to_string(format!("{}/app.json", path)).expect("Error reading App config");
  let eas = fs::read_to_string(format!("{}/eas.json", path)).expect("Error reading EAS config");
  let app_json: serde_json::Value = serde_json::from_str(&app_config).unwrap();
  let eas_json: serde_json::Value = serde_json::from_str(&eas).unwrap();
  let app_name = app_json["expo"]["name"].as_str().unwrap().replace("\"", "");
  info!("✅ EAS Configuration found for app: {}", app_name);
  let env = match eas_json["build"][profile]["env"].as_object() {
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
  return env.clone();
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
