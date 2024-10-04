pub mod utils {
    pub mod email;
}
pub mod collect_api;
pub mod collect_models;

#[macro_use] extern crate rocket;
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket::http::Method;



#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
        .allowed_methods(vec![Method::Post].into_iter().map(From::from).collect())
        .allow_credentials(true)
        .to_cors()
        .expect("ERROR AT CORS");

    
    rocket::build()
        .mount("/api", routes![collect_api::collect_data_api])
        .attach(cors)
}
