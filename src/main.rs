use std::fs::File;
use std::io::Read;
use std::process::Command;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    directory: String,
    commands: Vec<String>,
}

fn main() -> serde_json::Result<()> {
    let mut file = File::open("localstack.config.json").expect("Failed to open file");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Failed to read file");

    let configs: Vec<Config> = serde_json::from_str(&data)?;

    let mut children = Vec::new();

    for config in configs {
        for command in config.commands {
            let child = Command::new("cmd")
                .args(&["/C", "start", "cmd", "/K", &command])
                .current_dir(&config.directory)
                .spawn()
                .expect("Failed to execute command");
            children.push(child);
        }
    }
    Ok(())
}