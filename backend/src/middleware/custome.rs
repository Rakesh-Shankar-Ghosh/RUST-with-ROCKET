

// use rocket::{Request, Data};
// use rocket::data::{FromData, Outcome};
// use rocket::http::Status;
// use rocket::data::ByteUnit;
// use rocket::tokio::io::AsyncReadExt;


// #[derive(Debug)]
// struct CustomMiddleware;

// #[async_trait::async_trait]
// impl<'r> rocket::Data<'r> for CustomMiddleware {
//     type Error = std::io::Error;

//     async fn from_data(_req: &'r Request<'_>, data: Data<'r>) -> rocket::data::Outcome<'r, Self> {
//         // You can add your custom logic here
//         // For example, let's read the data and check if it contains "5 == 5"
//         let mut content = String::new();
//         if let Err(e) = data.open(ByteUnit::default()).read_to_string(&mut content).await {
//             return Outcome::Failure((Status::InternalServerError, e));
//         }

//         if content.trim() == "5 == 5" {
//             Outcome::Success(CustomMiddleware)
//         } else {
//             Outcome::Failure((Status::BadRequest, std::io::Error::new(std::io::ErrorKind::Other, "Not 5 == 5")))
//         }
//     }

//     fn default() -> ByteUnit {
//         ByteUnit::default()
//     }
// }

