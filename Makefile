install:
	cargo install cargo-edit
	cargo install sqlx-cli
	cargo add actix-files
	cargo add actix-web
	cargo add chrono
	cargo add sqlx --features postgres,runtime-tokio-rustls,runtime-tokio,macros,migrate
	cargo add tera
	cargo add dotenv
	cargo add serde --features derive
	cargo add tera
	cargo add anyhow

build:
	cargo build