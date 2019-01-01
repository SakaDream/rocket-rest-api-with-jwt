# Rocket REST API with JWT

A Rusty Rocket 🚀 fuelled with Diesel 🛢 and secured by JWT 🔐

## Require

- [Rust](rust-lang.org)
- [Postgres](https://www.postgresql.org/)

## How to run

- Install Rust Nightly: `rustup install nightly`
- Set Rust Nightly to project: Go to the root of the project, open cmd/terminal and run `rustup override set nightly`
- Rename `Rocket.toml.sample` to `Rocket.toml` and update the value in `url` key
- Rename `secret.key.sample` to `secret.key` or create an own key by running `head -c16 /dev/urandom > secret.key` in command line (Linux/UNIX only) and copy to `/src` folder
- Create a database in postgres cli or pgadmin tool
- Build and run project: `cargo run`
- Enjoy! 😄