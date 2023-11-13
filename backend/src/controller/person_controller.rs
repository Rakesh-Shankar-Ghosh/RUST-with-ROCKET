use rocket::{get};
use rocket::response::status::{Custom};
use rocket_contrib::json::Json;
use serde::Serialize;
use serde::Deserialize;
use diesel::prelude::*;

use crate::schema::persons;
use crate::helper::db_conn::DbConn;

use crate::model::person_model::Person;

use crate::route::person_route::MyGuard;


extern crate rusqlite;
use rusqlite::{Connection, Result};


use rusqlite::params;



#[put("/update_person/<id>", data = "<person_update>")]
pub fn update_person_by_id(id: i32, person_update: Json<Person>) -> Result<Json<String>> {
    // Open a connection to the SQLite database (consider using connection pooling)
    let conn = Connection::open("testing_db.db")?;

    // Prepare an SQL statement to update a person by ID
    let updated_person = Person {
        id: Some(id),
        name: person_update.name.clone(),
        email: person_update.email.clone(),
    };

    // Unwrap the id from Option<i32>
    let unwrapped_id = updated_person.id.unwrap_or_else(|| {
        // Handle the case where id is None (you can choose an appropriate default value)
        // For now, panic if id is None
        panic!("ID cannot be None");
    });

    // Execute the SQL UPDATE statement with parameters
    conn.execute(
        "UPDATE persons SET name = ?, email = ? WHERE id = ?",
        params![&updated_person.name, &updated_person.email, &unwrapped_id],
    )?;

    Ok(Json("Person updated successfully".to_string()))
}


#[delete("/delete_person/<id>")]
pub fn delete_person_by_id(id: i32) -> Result<Json<String>> {
    // Open a connection to the SQLite database (consider using connection pooling)
    let conn = Connection::open("testing_db.db")?;

    // Prepare an SQL statement to delete a person by ID
    let mut stmt = conn.prepare("DELETE FROM persons WHERE id = ?")?;

    // Execute the statement with the provided ID
    stmt.execute([id])?;

    // Return a JSON response indicating success
    Ok(Json("Person deleted successfully".to_string()))
}



#[get("/get_person/<id>")]
pub fn get_person_by_id(id: i32) -> Result<Json<Person>> {
    // Open a connection to the SQLite database (consider using connection pooling)
    let conn = Connection::open("testing_db.db")?;

    // Prepare an SQL statement to fetch a person by ID
    let mut stmt = conn.prepare("SELECT id, name, email FROM persons WHERE id = ?")?;

    // Execute the statement with the provided ID
    let person_row = stmt.query_row([id], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
        })
    });

    // Return the result as JSON
    Ok(Json(person_row?))
}



#[get("/get_allperson")]
pub fn get_allperson() -> Result<Json<Vec<Person>>> {
    // Open a connection to the SQLite database (consider using connection pooling)
    let conn = Connection::open("testing_db.db")?;

    // Prepare an SQL statement to fetch all persons
    let mut stmt = conn.prepare("SELECT id, name, email FROM persons")?;

    // Execute the statement and fetch all rows
    let person_rows = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
        })
    })?;

    // Collect the results into a vector
    let persons: Result<Vec<Person>> = person_rows.collect();

    // Return the result as JSON
    Ok(Json(persons?))
}


#[post("/add_person", data = "<person_input>")]
pub fn add_person(person_input: Json<Person>, mut conn: DbConn) -> Result<String, String> {
    let new_person = person_input.0; // Extract the inner Person from Json

    let result = diesel::insert_into(persons::table) // Use the schema's table reference
    .values(new_person)
    .execute(&mut *conn);

    match result {
        Ok(_) => Ok(format!("Inserted person successfully")),
        Err(err) => Err(format!("Error inserting person: {}", err)),
    }
}







//************test***************

// use rocket::{Route, Response, http::{Status, ContentType}, response::{Responder, status::Custom}, data::{FromData, ByteUnit, ByteUnit, ByteUnit, ByteUnit}};
// use rocket::data::ByteUnit;
// use rocket::data::ByteUnit;
// use rocket::data::ByteUnit;
// use rocket::data::ByteUnit;
// use rocket::data::ByteUnit;
// use rocket::data::ByteUnit;

// use crate::model::student_model;

// #[derive(Serialize)]
// pub struct ResponseData {
//     success: bool,
//     message: String,
//     data: Option<Person>,
// }
// #[post("/addPerson", data = "<person_data>")]
// pub fn addPerson(person_data: Form<PersonFormData>, connection: DbConn) -> Custom<Json<ResponseData>> {
//     let result = std::panic::catch_unwind(|| {
//         let new_person = student_model::Person {
//             id: None,
//             name: person_data.name.clone(),
//             email: person_data.email.clone(),
//         };

//         if new_person.insert(&connection).await {
//             ResponseData {
//                 success: true,
//                 message: "Person added successfully!".to_string(),
//                 data: Some(new_person),
//             }
//         } else {
//             ResponseData {
//                 success: false,
//                 message: "Failed to add person.".to_string(),
//                 data: None,
//             }
//         }
//     });

//     match result {
//         Ok(response_data) => {
//             Custom(
//                 rocket::http::Status::new(200, "OK"),
//                 Json(response_data),
//             )
//         }
//         Err(_) => {
//             let error_response = ResponseData {
//                 success: false,
//                 message: "An error occurred".to_string(),
//                 data: None,
//             };
//             Custom(
//                 rocket::http::Status::new(500, "Internal Server Error"),
//                 Json(error_response),
//             )
//         }
//     }
// }

//***************************




#[derive(Serialize)]
pub struct ResponseData {
    success: bool,
    message: String,
    data: Option<i32>,
}

#[get("/index")]
pub fn index() -> Custom<Json<ResponseData>> {
    let result = std::panic::catch_unwind(|| {
        let data = 5;
        if data == 5 {
            ResponseData {
                success: true,
                message: "perfectly done".to_string(),
                data: Some(data),
            }
        } else {
            ResponseData {
                success: false,
                message: "failed".to_string(),
                data: None,
            }
        }
    });

    match result {
        Ok(response_data) => {
            Custom(
                rocket::http::Status::new(200, "OK"),
                Json(response_data),
            )
        }
        Err(_) => {
            let error_response = ResponseData {
                success: false,
                message: "An error occurred".to_string(),
                data: None,
            };
            Custom(
                rocket::http::Status::new(500, "Internal Server Error"),
                Json(error_response),
            )
        }
    }
}


#[get("/hello")]
pub fn hello(filter: MyGuard) -> &'static str {
    // The `MyGuard` is applied to this route, so if it reaches this point,
    // the guard condition is met.
    "Hello, World!"
}

#[get("/bye")]
pub fn bye() -> &'static str {
    "Goodbye, Rocket!"
}






// pub fn say_hello()  {
//     println!("Hello, World! Controller");
// }

