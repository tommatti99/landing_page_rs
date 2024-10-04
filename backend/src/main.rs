pub mod utils {
    pub mod email;
}
pub mod collect_api;
pub mod collect_models;

#[macro_use] extern crate rocket;
use rocket_cors::{CorsOptions, AllowedOrigins};
use rocket::http::Method;



#[launch]
fn rocket() -> _ {
    
    let allowed_origins = AllowedOrigins::some(&[
        "https://landing-page-rs.onrender.com".into(),
    ]);

    let cors = CorsOptions::default()
        .allowed_origins(allowed_origins)
        .allowed_methods(vec![Method::Post, Method::Get].into_iter().map(From::from).collect())
        .allow_credentials(true)
        .allowed_headers(rocket_cors::AllowedHeaders::all())
        .to_cors()
        .expect("ERROR AT CORS");

    
    rocket::build()
        .mount("/api", routes![collect_api::collect_data_api])
        .attach(cors)
}
