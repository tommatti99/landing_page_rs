pub mod utils {
    pub mod email;
}
pub mod collect_api;
pub mod collect_models;

#[macro_use] extern crate rocket;
use rocket_cors::{CorsOptions, AllowedOrigins};
use rocket::http::Method;
use rocket::figment::Figment;


#[launch]
fn rocket() -> _ {
    
    let allowed_origins = AllowedOrigins::some::<&str, &str>(
        &["https://landing-page-rs-backend.up.railway.app/".into()],
        &[] 
    );

    let figment: Figment = Figment::from(rocket::Config::default())
        .merge(("port", std::env::var("PORT").unwrap().parse::<u16>().unwrap()))
        .merge(("address", "0.0.0.0"))
        .merge(("email_receiver", std::env::var("EMAIL_RECEIVER").unwrap()))
        .merge(("smtp_email", std::env::var("SMTP_EMAIL").unwrap()))
        .merge(("smtp_host", std::env::var("SMTP_HOST").unwrap()));

    let cors = CorsOptions::default()
        .allowed_origins(allowed_origins)
        .allowed_methods(vec![Method::Post, Method::Get].into_iter().map(From::from).collect())
        .allow_credentials(true)
        .allowed_headers(rocket_cors::AllowedHeaders::all())
        .to_cors()
        .expect("ERROR AT CORS");

    
    rocket::custom(figment)
        .mount("/api", routes![collect_api::collect_data_api])
        .attach(cors)
}
