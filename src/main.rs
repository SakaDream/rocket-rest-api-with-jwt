#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate bcrypt;
extern crate dotenv;
extern crate jsonwebtoken;
extern crate serde;
extern crate serde_json;
extern crate time;

mod api;
mod config;
mod constants;
mod jwt;
mod models;
mod schema;
mod services;

fn main() {
    config::rocket().0.launch();
}
