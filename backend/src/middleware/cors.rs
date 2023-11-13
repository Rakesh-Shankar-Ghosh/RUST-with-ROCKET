// cors.rs

use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors};
use rocket::http::Method;

pub fn cors_options() -> Cors {
    let allowed_origins = AllowedOrigins::all();

    Cors {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
}
