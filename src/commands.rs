use crate::state::State;
use std::{
    path::Path,
    process::{Command, Stdio},
};

use sysinfo::{Pid, System};

pub fn up(filename: &Path, mut s: State) {
    let fname = String::from(filename.file_name().unwrap().to_str().unwrap_or(""));
    match Command::new(filename).stdout(Stdio::null()).spawn() {
        Ok(child) => {
            s.add(fname.clone(), child.id());
            println!("\n💅 {} started successfully. PID: {}\n", fname, child.id());
        }
        Err(err) => {
            println!("\n🔥 Failed to start child process: {}\n", err);
        }
    }
    let mut sys = System::new_all();
    sys.refresh_all();
    refresh_processes(s, sys, true);
}

pub fn down(filename: &String, mut s: State) {
    let mut sys = System::new_all();
    sys.refresh_all();
    for (pid, _) in s.get().clone() {
        match sys.process(Pid::from_u32(pid.clone())) {
            Some(p) => {
                if p.name().eq(filename.as_str()) {
                    p.kill();
                    s.remove(pid);
                    break;
                }
            }
            None => {}
        }
    }

    refresh_processes(s, sys, true);
}

pub fn status(s: State) {
    let mut sys = System::new_all();
    sys.refresh_all();
    refresh_processes(s, sys, true);
}

fn get_ports(pid: u32) -> String {
    let output = Command::new("lsof")
        .args(&["-i", "-n", "-P", "-a", "-p", &pid.to_string()])
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut result = String::new();

        for line in stdout.lines() {
            if line.contains("TCP") {
                let parts: Vec<_> = line.split_whitespace().collect();
                if let Some(local_port) = parts.get(8) {
                    result.push_str(local_port);
                }
            }
        }

        result
    } else {
        String::new()
    }
}

fn refresh_processes(mut s: State, sys: System, print: bool) {
    let processes = s.get().clone();
    if print {
        if processes.len() > 0 {
            println!("\nName\t\tPID\tPorts\tCPU\tMemory");
        } else {
            println!("\n💅 Nothing running. Go make something fab!");
        }
    }
    for (pid, name) in processes {
        match sys.process(Pid::from_u32(pid.clone())) {
            Some(p) => {
                if p.name().eq(&name) {
                    let ports = get_ports(pid);
                    if print {
                        println!(
                            "{}\t{}\t{}\t{}\t{}",
                            name,
                            pid,
                            ports,
                            p.cpu_usage(),
                            p.memory() / 1_024 / 1_024
                        )
                    }
                } else {
                    s.remove(pid.clone())
                }
            }
            None => s.remove(pid.clone()),
        }
    }
}
