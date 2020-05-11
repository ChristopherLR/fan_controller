use serde::{Deserialize, Serialize};
use serde_yaml;
use std::env;
use std::process::Command;
use crate::FanMode::Automatic;
use crate::FanMode::Manual;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Env {
    host: String,
    user: String,
    password: String,
}

enum FanMode {
    Manual,
    Automatic,
}

fn change_speed(speed: u32) -> String {
    if speed > 4000 {
				println!("Speed: 0x10 - 3000RPM");
        "raw 0x30 0x30 0x02 0xff 0x10".to_string()
    } else if speed > 2000 && speed < 3000 {
				println!("Speed: 0x0a - 2160RPM");
        "raw 0x30 0x30 0x02 0xff 0x0a".to_string()
    } else if speed < 2000 {
				println!("Speed: 0x09 - 1560RPM");
        "raw 0x30 0x30 0x03 0xff 0x09".to_string()
    } else {
				println!("Speed: 0x09 - 2160RPM");
        "raw 0x30 0x30 0x02 0xff 0x0a".to_string()
    }
}

fn fan_mode(fm: FanMode) -> String {
    match fm {
        Manual => {
						println!("Manual fanmode");
						"raw 0x30 0x30 0x01 0x00".to_string()
				},
        Automatic => {
						println!("Automatic fanmode");
						"raw 0x30 0x30 0x01 0x01".to_string()
				},
    }
}

fn ipmitool_send(req : String, env : Env) {
    let mut echo = Command::new("sh");

		match echo.output() {
        Ok(o) => unsafe {
            println!(
                "Out: {}, args: {:?}, env: {}",
                String::from_utf8_unchecked(o.stdout),
                req,
                serde_yaml::to_string(&env).unwrap()
            );
        },
        Err(e) => {
            println!("did not work: {}", e);
        }
    }
}

fn main() {
    let default_conf = Env {
        host: String::from("uninit"),
        user: String::from("uninit"),
        password: String::from("uninit"),
    };

    let args: Vec<String> = env::args().collect();
    let mut env = default_conf;

    match std::fs::File::open("./env.yaml") {
        Ok(o) => match serde_yaml::from_reader(o) {
            Ok(out) => env = out,
            Err(e) => println!("Yaml Parse: {}", e),
        },
        Err(e) => println!("Open File: {}", e),
    }

    if env.password == "uninit" {
        panic!("env.yaml password not defined")
    }

    if env.host == "uninit" {
        panic!("env.yaml host not defined")
    }

    if env.user == "uninit" {
        panic!("env.yaml user not defined")
    }

		let mut resp : String = "uninit".to_string();

		if args.len() > 2 {
				if args[1] == "speed" {
						println!("Using speed");
						match args[2].parse::<u32>(){
								Ok(n) => resp = change_speed(n),
								Err(e) => resp = change_speed(3000)
						}
				}
		} else if args.len() == 2 {
				if args[1] == "man" {
						resp = fan_mode(Manual)
				} else {
						resp = fan_mode(Automatic)
				}
		}

		if resp == "uninit" {
				println!("No parameters were parsed correctly");
		} else {
				ipmitool_send(resp, env)
		}

}
