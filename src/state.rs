use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;

type Processes = HashMap<u32, String>;

#[derive(Serialize, Deserialize)]
pub struct State {
    processes: Processes,
    config: PathBuf,
}

impl State {
    pub fn new() -> Self {
        let config = State::get_config();
        let processes = match State::deserialize(&config) {
            Some(processes) => processes,
            None => HashMap::new(),
        };
        State { config, processes }
    }

    pub fn get(&self) -> &Processes {
        &self.processes
    }

    pub fn add(&mut self, filename: String, pid: u32) {
        self.processes.insert(pid, filename);
        match File::create(&self.config) {
            Ok(mut file) => {
                if let Err(e) = serde_json::to_writer_pretty(&mut file, &self.processes) {
                    eprintln!("Error writing to config file: {:?}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to create config file: {:?}", e);
            }
        }
    }

    pub fn remove(&mut self, pid: u32) {
        self.processes.remove(&pid);
        let mut file = File::create(&self.config).unwrap();
        let _ = serde_json::to_writer_pretty(&mut file, &self.processes);
    }

    fn deserialize(path: &PathBuf) -> Option<Processes> {
        let file = File::open(&path);
        if let Ok(mut file) = file {
            let mut contents = String::new();
            let _ = file.read_to_string(&mut contents);

            if let Ok(data) = serde_json::from_str::<Processes>(&contents) {
                return Some(data);
            }
        }
        return None;
    }

    fn get_config() -> PathBuf {
        let dirs = ProjectDirs::from("com", "fab", "fab-cli");
        let dir = match dirs {
            Some(d) => d.config_dir().to_path_buf(),
            None => env::current_dir().unwrap(),
        };
        if let Err(e) = fs::create_dir_all(&dir) {
            println!("Failed to create directory: {}", e);
        }
        println!("Config: {:?}", &dir);
        Path::join(&dir, "config.json")
    }
}
