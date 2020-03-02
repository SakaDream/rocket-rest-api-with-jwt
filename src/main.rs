#![feature(proc_macro_hygiene, decl_macro)]
#![allow(proc_macro_derive_resolution_fallback)]

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
extern crate chrono;
extern crate uuid;

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
