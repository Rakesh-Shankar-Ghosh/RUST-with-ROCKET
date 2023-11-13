

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod helper;
use helper::db_conn::{DbConn,establish_connection,test_connection};

mod model;
use model::person_model::Person;

mod route;
use route::person_route;

mod controller;
use controller::person_controller;

mod schema;
use schema::persons;

mod middleware;
use middleware::{cors::cors_options};

mod filter;

use rocket_contrib::json::Json;
use serde::Deserialize;
use diesel::prelude::*;







fn main() {


    let cors = cors_options(); //middleware
    
    establish_connection();
    test_connection();

   
    rocket::ignite()
        .attach(cors)
        .mount("/api/v1", person_route::person_route()).launch();
        
}



