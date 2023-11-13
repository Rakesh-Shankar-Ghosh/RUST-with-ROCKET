use diesel::prelude::*;
use serde::Serialize;
use serde::Deserialize;
use diesel::prelude::*;

use crate::schema::persons;

// #[derive(Debug, PartialEq, Eq, Deserialize, Insertable ,Queryable, Selectable)]
// pub struct Person {
//     pub id: Option<i32>,
//     pub name: String,
//     pub email: String,
    
// }


#[derive(Debug, PartialEq, Eq, Serialize,Queryable, Selectable,Insertable, Deserialize)]
pub struct Person {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
}
