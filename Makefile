install:
	cargo build --release
	cp target/release/fan_controller /usr/local/bin
