use crate::utils::email::{self, EmailMessage};
use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CollectDataRequest {
    pub name: String,
    pub surname: String,
    pub prefix_telephone_number: String,
    pub telephone_number: String,
    pub cpf: String,
    pub email: String,
    pub birth: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub banco: String,
    pub agencia: String,
    pub conta: String,
    pub digito_verif: String,
    pub terms_and_cond: String
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
