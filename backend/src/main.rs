pub mod utils {
    pub mod email;
}
pub mod collect_api;
pub mod collect_models;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![collect_api::collect_data_api])
}
