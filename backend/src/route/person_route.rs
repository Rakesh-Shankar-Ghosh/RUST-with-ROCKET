
use rocket::{Route};

use crate::controller::person_controller;


use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

pub struct MyGuard;

impl<'a, 'r> FromRequest<'a, 'r> for MyGuard {
    type Error = ();

    fn from_request(_req: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        if 5 == 5 {
            Outcome::Success(MyGuard)
        } else {
            Outcome::Failure((Status::Forbidden, ()))
        }
    }
}




pub fn person_route() -> Vec<Route> {
    routes![
        person_controller::hello,
        person_controller::bye,
        person_controller::index,
        person_controller::add_person,
        person_controller::get_allperson,
        person_controller::get_person_by_id,
        person_controller::delete_person_by_id,
        person_controller::update_person_by_id,

     

    ]
}








// use crate::controller::person_controller;

// pub fn say_hello()  {
//     println!("Hello, World! Routesss");
//     person_controller::say_hello();
// }

