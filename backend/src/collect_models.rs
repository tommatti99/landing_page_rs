use crate::utils::email::{self, EmailMessage}

pub struct CollectDataRequest {
    name: String,
    telephone_number: String,
    email: String,
    want_to_receive_more_info: bool,
    already_have_the_product: bool
}

pub struct CollectDataResponse {
    success: bool
}

impl CollectDataResponse {
    pub fn send_info(data: CollectDataRequest) -> bool {
        return email::send_email(EmailMessage::landing_page_email(data));
    } 
}