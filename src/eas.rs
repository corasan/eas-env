use clap::Parser;
use colored::*;
use env_logger::Builder;
use log::LevelFilter;
use log::{error, info};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
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
  let dir = args
    .app_dir
    .unwrap_or_else(|| env::current_dir().expect("Failed to get current directory"));
  match write_to_env(&profile, &dir) {
    Ok(_) => info!(
      "✅ Environment file created successfully using profile: \"{}\".\n",
      profile
    ),
    Err(e) => error!("Failed to create environment file: {}", e),
  }
}

fn write_to_env(profile: &str, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
  let eas_env = get_eas_env(profile, path)?;
  let file_path = path.join(".env.local");
  let file = fs::File::create(&file_path)?;

  let mut file = BufWriter::new(file);
  for (key, value) in eas_env {
    writeln!(file, "{}={}", key, value)?;
  }
  Ok(())
}

fn read_and_parse_json(
  path: &Path,
  file_name: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
  let file_content = fs::read_to_string(path.join(file_name))?;
  let json: serde_json::Value = serde_json::from_str(&file_content)?;
  Ok(json)
}

fn get_eas_env(
  profile: &str,
  path: &Path,
) -> Result<serde_json::Map<String, serde_json::Value>, Box<dyn std::error::Error>> {
  let app_json = read_and_parse_json(path, "app.json")?;
  let eas_json = read_and_parse_json(path, "eas.json")?;
  let app_name = app_json["expo"]["name"]
    .as_str()
    .ok_or("Expected a string")?
    .replace("\"", "");
  info!("✅ EAS Configuration found for app: {}", app_name);
  let env = match eas_json["build"][profile]["env"].as_object() {
    Some(env) => env,
    None => {
      error!("\"env\" field not found");
      return Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "\"env\" field not found",
      )));
    }
  };
  if env.is_empty() {
    error!("\"env\" field has no keys");
    return Err(Box::new(std::io::Error::new(
      std::io::ErrorKind::InvalidData,
      "\"env\" field not found",
    )));
  }
  Ok(env.clone())
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
