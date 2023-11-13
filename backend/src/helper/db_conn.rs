use diesel::sqlite::SqliteConnection;
use diesel::Connection;
use dotenv::dotenv;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::Outcome::*;
use rocket::http::Status;
use std::env;
use std::ops::{Deref, DerefMut}; // Import the Deref and DerefMut traits

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    SqliteConnection::establish(&database_url).expect("Error connecting to the database")
  
}

pub fn test_connection() -> bool {
    dotenv().ok(); // Load environment variables from .env file

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");
    
    match SqliteConnection::establish(&database_url) {
        Ok(_) => {
            println!("Connected to database: {}", database_url);
            true
        }
        Err(err) => {
            eprintln!("Error connecting to {}: {}", database_url, err);
            false
        }
    }
}

pub struct DbConn(pub SqliteConnection);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(_request: &'a Request<'r>) -> rocket::request::Outcome<Self, Self::Error> {
        let conn = establish_connection();
        Success(DbConn(conn))
    }
}

impl Deref for DbConn {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DbConn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


