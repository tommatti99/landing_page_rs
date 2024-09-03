use std::fs;
use crate:: collect_models::CollectDataRequest;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

fn secrets(path: &str) -> String {
    return fs::read_to_string(path).map(|s| s.trim().to_string()).unwrap();
}


pub struct EmailMessage {
    pub email_to: String,
    pub email_from: String,
    pub email_subject: String,
    pub email_body: String,
}

impl EmailMessage {
    pub fn landing_page_email(data: CollectDataRequest) -> Self {
        let email_receiver = secrets("/run/secrets/email_receiver")
        let smtp_email = secrets("/run/secrets/smtp_email")

        EmailMessage {
            email_to:email_receiver,
            email_from: smtp_email,
            email_subject: "Landing Page".to_string(),
            email_body: format!(
                "New Client \n\n

                 name: {}\n
                 telephone_number: {}\n
                 email: {}\n
                 want_to_receive_more_info: {}\n
                 already_have_the_product: {}\n",
                    data.name, data.telephone_number, data.email, 
                    data.want_to_receive_more_info, data.already_have_the_product)
        }
    }
}

fn open_conn_email() -> SmtpTransport {
    let smtp_email = secrets("/run/secrets/smtp_email")
    let smtp_key = secrets("/run/secrets/smtp_key")
    let smtp_host = secrets("/run/secrets/smtp_host")
    
    let admin_cred: Credentials = Credentials::new(
        (smtp_email).to_string().to_owned(),
        (smtp_key).to_owned(),
    );

    return SmtpTransport::relay(smtp_host.expect("").clone())
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