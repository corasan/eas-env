use std::{env, fs};
use std::io::{BufWriter, Write};
use serde_json::Value;

pub fn create_env() {
  let args = env::args().collect::<Vec<String>>();
  if args.len() < 2 {
    write_to_env("default".to_string());
    return;
  }
  write_to_env(args[1].to_string());
  print!("Done ✅\n")
}

fn write_to_env(profile: String) {
  let eas = read_config();
  let p = profile.clone();
  let env = match eas["build"][profile]["env"].as_object() {
      Some(env) => env,
      None => {
          eprintln!("Error: 'env' field not found");
          std::process::exit(1);
      }
  };
  if env.is_empty() {
      eprintln!("Error: 'env' field has no keys");
      std::process::exit(1);
  }
  let dir = get_dir();
  let file = fs::File::create(format!("{}/.env.local", dir)).expect("Error creating .env file");
  println!("[LOG] Creating .env.local with profile: {}", p);
  let mut file = BufWriter::new(file);
  for (key, value) in env {
      write!(file, "{}={}\n", key, value).expect("Error writing to file");
  }
}

fn read_config() -> Value {
  let dir = get_dir();
  let app_config = fs::read_to_string(format!("{}/app.json", dir)).expect("Error reading App config");
  let eas = fs::read_to_string(format!("{}/eas.json", dir)).expect("Error reading EAS config");
  let app_json: serde_json::Value = serde_json::from_str(&app_config).unwrap();
  let eas_json: serde_json::Value = serde_json::from_str(&eas).unwrap();
  let app_name = app_json["expo"]["name"].as_str().unwrap().replace("\"", "");
  println!("[LOG] EAS Configuration found for app: {}", app_name);
  return eas_json;
}

fn get_dir() -> String {
  let current_dir = env::current_dir().expect("Error getting current directory");
  let string = current_dir.to_str().expect("Error converting path to string");
  return string.to_string();
}
