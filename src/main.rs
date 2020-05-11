use std::process::Command;
use std::env;

fn main() {
		let args: Vec<String> = env::args().collect();
		let mut echo = Command::new("sh");
		echo.arg("-c")
				.arg("echo hello");
		match echo.output() {
				Ok(o) => {
						unsafe {
								println!("Out: {}, args: {:?}", String::from_utf8_unchecked(o.stdout), args);
						}
				},
				Err(e) => {
						println!("did not work: {}", e);
				}
		}
}
