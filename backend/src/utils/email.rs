use std::env;
use dotenv::dotenv;
use crate:: collect_data_models::CollectDataRequest;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};


const SMTP_KEY = env::var("smtp_key").expect("")
const SMTP_EMAIL = env::var("smtp_email").expect("")
const SMTP_HOST = env::var("smtp_host").expect("")
const EMAIL_RECEIVER = env::var("email_receiver").expect("")


pub struct EmailMessage {
    pub email_to: String,
    pub email_from: String,
    pub email_subject: String,
    pub email_body: String,
}

impl EmailMessage {
    pub fn landing_page_email(data: CollectDataRequest) -> Self {
        EmailMessage {
            email_to: EMAIL_RECEIVER,
            email_from: SMTP_EMAIL,
            email_subject: "Landing Page",
            email_body: format!(
                "New Client \n\n

                 name: {}\n
                 telephone_number: {}\n
                 email: {}\n
                 want_to_receive_more_info: {}\n
                 already_have_the_product: {}\n",
                    data.name, data.telephone_number, data.email, 
                    data.want_to_receive_more_info, data.already_have_the_product);
        }
    }
}

fn open_conn_email() -> SmtpTransport {
    let admin_cred: Credentials = Credentials::new(
        (SMTP_EMAIL).to_string().to_owned(),
        (SMTP_KEY).to_owned(),
    );

    return SmtpTransport::relay(&SMTP_HOST.clone())
        .unwrap()
        .credentials(admin_cred)
        .build();
}

pub fn send_email(msg_structure: EmailMessage) -> bool {
    let smtp_transp: SmtpTransport = open_conn_email();

    let email_msg = Message::builder()
        .from(msg_structure.email_from.parse().unwrap())
        .to(msg_structure.email_to.parse().unwrap())
        .subject(msg_structure.email_subject)
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(msg_structure.email_body.clone()))
        .unwrap();

    match smtp_transp.send(&email_msg) {
        Ok(_) => {
            return true;
        }       
        Err(_) => {
            return false;
        }
    }
}