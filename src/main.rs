use std::{env, fs};
use std::io::{BufWriter, Write};
use serde_json::Value;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        write_to_env("default".to_string());
        return;
    }
    write_to_env(args[1].to_string());
    print!("Done ✅")
}

fn write_to_env(profile: String) {
    let eas = read_config();
    let file = fs::File::create(format!("{}/.env.local", get_dir())).expect("Error creating .env file");
    println!("Creating .env.local with profile {}", profile);
    let mut file = BufWriter::new(file);
    for (key, value) in eas["build"][profile]["env"].as_object().unwrap() {
        write!(file, "{}={}\n", key, value).expect("Error writing to file");
    }
}

fn read_config() -> Value {
    let app_config = fs::read_to_string(format!("{}/app.json", get_dir())).expect("Error reading App config");
    let eas = fs::read_to_string(format!("{}/eas.json", get_dir())).expect("Error reading EAS config");
    let app_json: serde_json::Value = serde_json::from_str(&app_config).unwrap();
    let eas_json: serde_json::Value = serde_json::from_str(&eas).unwrap();
    println!("EAS Configuration found for app {}", app_json["expo"]["name"]);
    return eas_json;
}

fn get_dir() -> String {
    let current_dir = env::current_dir().expect("Error getting current directory");
    let string = current_dir.to_str().expect("Error converting path to string");
    return string.to_string();
}

