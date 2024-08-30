use crate::utils::email::{self, EmailMessage};
use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CollectDataRequest {
    pub name: String,
    pub telephone_number: String,
    pub email: String,
    pub want_to_receive_more_info: bool,
    pub already_have_the_product: bool
}

#[derive(Serialize, Deserialize)]
pub struct CollectDataResponse {
    pub success: bool
}

impl CollectDataResponse {
    pub fn send_info(data: CollectDataRequest) -> Self {
        return 
            CollectDataResponse {
                success: email::send_email(EmailMessage::landing_page_email(data))
            }
    }       
}