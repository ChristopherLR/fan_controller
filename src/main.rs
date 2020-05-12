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

fn change_speed(speed: u32) -> Vec<String> {
		let mut retv = Vec::new();
		retv.push(String::from("raw"));
		retv.push(String::from("0x30"));
		retv.push(String::from("0x30"));
		retv.push(String::from("0x02"));
		retv.push(String::from("0xff"));
    if speed > 4000 {
				println!("Speed: 0x10 - 3000RPM");
        retv.push(String::from("0x10"));
    } else if speed > 2000 && speed < 3000 {
				println!("Speed: 0x0a - 2160RPM");
        retv.push(String::from("0x0a"));
    } else if speed < 2000 {
        retv.push(String::from("0x09"));
				println!("Speed: 0x09 - 1560RPM");
    } else {
        retv.push(String::from("0x0a"));
				println!("Speed: 0x09 - 2160RPM");
    }
		retv
}

fn fan_mode(fm: FanMode) -> Vec<String> {
		let mut retv = Vec::new();
		retv.push(String::from("raw"));
		retv.push(String::from("0x30"));
		retv.push(String::from("0x30"));
		retv.push(String::from("0x01"));
    match fm {
        Manual => {
						println!("Manual fanmode");
						retv.push(String::from("0x00"));
				},
        Automatic => {
						println!("Automatic fanmode");
						retv.push(String::from("0x01"));
				},
    }
		retv
}

fn ipmitool_send(req : Vec<String>, env : Env) {
    let mut fan = Command::new("ipmitool");

		println!("ipmitool -I lanplus -H {} -U {} -P {} {}", env.host, env.user, env.password, req.join(" "));
		fan.arg("-I")
				.arg("lanplus")
				.arg("-H")
				.arg(env.host)
				.arg("-U")
				.arg(env.user)
				.arg("-P")
				.arg(env.password)
				.args(&req);

		fan.spawn().expect("No hostname specified!");
}

fn get_temp() -> Vec<String> {
		let mut retv = Vec::new();

		retv.push(String::from("sdr"));
		retv.push(String::from("type"));
		retv.push(String::from("temperature"));

		retv
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

		let mut resp = Vec::new();

		if args.len() > 2 {
				if args[1] == "speed" {
						println!("Using speed");
						match args[2].parse::<u32>(){
								Ok(n) => resp = change_speed(n),
								Err(_) => resp = change_speed(3000)
						}
				}
		} else if args.len() == 2 {
				if args[1] == "man" {
						resp = fan_mode(Manual)
				} else if args[1] == "temp" {
						resp = get_temp();
				} else {
						resp = fan_mode(Automatic)
				}
		}

		if resp.len() < 1{
				println!("No parameters were parsed correctly");
		} else {
				ipmitool_send(resp, env)
		}
}
