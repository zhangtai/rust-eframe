reinstall:
	cargo build --release
	sudo cp target/release/eframe /usr/local/bin
