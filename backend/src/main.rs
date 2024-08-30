#[macro_use] extern crate rocket;
use crate::collect_data_api;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![collect_data_api::collect_data_api])
}
