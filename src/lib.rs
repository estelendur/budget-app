#![recursion_limit="128"]
#![feature(plugin, custom_derive, custom_attribute, try_trait)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_infer_schema;
#[macro_use] extern crate serde_derive;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate dotenv;
extern crate uuid;
extern crate bigdecimal;
extern crate chrono;
extern crate num_traits;
extern crate serde;
#[macro_use] extern crate serde_json;

mod schema;
mod models;
mod controllers;
mod database;
mod context;
mod error;

use dotenv::dotenv;
use std::env;
use rocket::Rocket;
use rocket_contrib::Template;

pub fn ignite() -> Rocket {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let db_pool = database::make_pool(&database_url);

    let r = rocket::ignite()
        .manage(db_pool)
        .attach(Template::fairing());
    
    controllers::mount(r)
}

pub fn start() {
    ignite().launch();
}
