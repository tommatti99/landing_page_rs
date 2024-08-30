use yew::prelude::*;
use reqwest::{Error, header};


const LANDING_PAGE_API = "/api/landing_page".to_string()

struct LandingPageRequest {
    name: String,
    telephone_number: String, 
    email: String,
    want_to_receive_more_info: bool,
    already_have_the_product: bool

} 
struct LandingPageResponse {
    status: bool
}


#[function_component(form)]
pub fn Form() -> Html {
    return html!{
        <div>
            <input type="text" id="name" />
            <input type="text" id="telephone_number" />
            <input type="text" id="email" />
            <input type="text" id="want_to_receive_more_info" />
            <input type="checkbox" ckecked=false id="already_have_the_product" />
        </div>
    };
}

async fn send_data() -> Result {
    let response = client
}