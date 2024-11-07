use std::collections::HashMap;
use yew::prelude::*;
use reqwest;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;

struct LandingPageRequest {
    name: String,
    telephone_number: String, 
    email: String,
    already_have_the_product: String,
    want_to_receive_more_info: bool
} 
#[derive(Deserialize, Debug)]
struct LandingPageResponse {
    status: bool
}


#[function_component]
pub fn Form() -> Html {

    let name = use_state(|| String::new());
    let telephone_number = use_state(|| String::new());
    let email = use_state(|| String::new());
    let already_have_the_product = use_state(|| String::new());
    let res_window_msg = use_state(|| String::new());
    let want_to_receive_more_info = use_state(|| false);
    let res_window_state = use_state(|| false);
    
    let name_input = {
        let name = name.clone();

        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                name.set(input.value());
        }
        })
    };

    let telephone_number_input = {
        let telephone_number = telephone_number.clone();
        
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                telephone_number.set(input.value());
        }
        })
    };
    let email_input = {
        let email = email.clone();
    
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                email.set(input.value());
        }
        })
    };

    let already_have_the_product_input = {
        let already_have_the_product = already_have_the_product.clone();
    
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                already_have_the_product.set(input.value());
        }
        })
    };
    
    let want_to_receive_more_info_input = {
        let want_to_receive_more_info = want_to_receive_more_info.clone();
    
        Callback::from(move |e: MouseEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                want_to_receive_more_info.set(input.checked());
        }
        })
    };

    let res_window_state_toggle: Callback<MouseEvent> = {
        let res_window_state = res_window_state.clone();
        Callback::from(move |_| res_window_state.set(!*res_window_state))
    };

    let click_submit = {
        let state_clone_name = name.clone();
        let state_clone_telephone_number = telephone_number.clone();
        let state_clone_email = email.clone();
        let state_clone_already_have_the_product = already_have_the_product.clone();
        let state_clone_want_to_receive_more_info = want_to_receive_more_info.clone();

        Callback::from(move |_: MouseEvent| {
            let request: LandingPageRequest = LandingPageRequest {
                name: (*state_clone_name).clone(),
                telephone_number: (*state_clone_telephone_number).clone(),
                email: (*state_clone_email).clone(),
                already_have_the_product: (*state_clone_already_have_the_product).clone(),
                want_to_receive_more_info: (*state_clone_want_to_receive_more_info).clone()
            };

            spawn_local(async move {
                let msg = send_request_to_api(request).await;
                res_window_msg.set(msg);
                res_window_state.set(true);
            });
        })
    };

    return html!{
        <div style={format!("display: flex; justify-content: center; align-items: center; height: 100vh; color: white;")}>
            <div style={format!("background-color: rgba(255,255,255,0.1); backdrop-filter: blur(8px); width:23em; border-radius: 0.5em")}>
            <div style={format!("flex-direction: column; align-items: center; display: flex; width: 20em; padding: 1em; border-radius: 8px;")}>
                <div style={format!("width: 100%; margin-bottom: 1em;")}>
                    <h2 style={format!("margin: 0.5em 0; font-size: 1em;")}>{"Name"}</h2>
                    <input type="text" id="name" value={(*name).clone()} oninput={name_input} style={format!("width: 100%; padding: 0.5em; border-radius: 4px; border: 1px solid #ccc;")} />
                </div>
                <div style={format!("width: 100%; margin-bottom: 1em;")}>
                    <h2 style={format!("margin: 0.5em 0; font-size: 1em;")}>{"Telephone number"}</h2>
                    <input type="text" id="telephone_number" value={(*telephone_number).clone()} oninput={telephone_number_input} style={format!("width: 100%; padding: 0.5em; border-radius: 4px; border: 1px solid #ccc;")} />
                </div>
                <div style={format!("width: 100%; margin-bottom: 1em;")}>
                    <h2 style={format!("margin: 0.5em 0; font-size: 1em;")}>{"Email address"}</h2>
                    <input type="text" id="email" value={(*email).clone()} oninput={email_input} style={format!("width: 100%; padding: 0.5em; border-radius: 4px; border: 1px solid #ccc;")} />
                </div>
                <div style={format!("width: 100%; margin-bottom: 1em;")}>
                    <h2 style={format!("margin: 0.5em 0; font-size: 1em;")}>{"Do you already have our products? Which?"}</h2>
                    <input type="text" id="already_have_the_product" value={(*already_have_the_product).clone()} oninput={already_have_the_product_input} style={format!("width: 100%; padding: 0.5em; border-radius: 4px; border: 1px solid #ccc;")} />
                </div>
                <div style={format!("width: 100%; margin-bottom: 1em;")}>
                    <h2 style={format!("margin: 0.5em 0; font-size: 1em;")}>{"Do you want to receive more information about our products?"}</h2>
                <input type="checkbox" id="want_to_receive_more_info" checked={*want_to_receive_more_info} onclick={want_to_receive_more_info_input}  style={format!("transform: scale(1.5);")} />
                </div>
                <button onclick = {click_submit} style={format!("padding: 0.5em 1em; background-color: white; color: black; border: none; border-radius: 4px; cursor: pointer; font-size: 1em;")}>
                {"Submit"}
                </button>
            </div>
        </div>
        {if *res_window_state {
                html!{
                    <>
                        <div class ="result_box">
                            <h2>{ (*res_window_msg).clone() }</h2>
                            <button onclick = {res_window_state_toggle} style={format!("padding: 0.5em 1em; background-color: white; color: black; border: none; border-radius: 4px; cursor: pointer; font-size: 1em;")}>
                                {"Ok!"}
                            </button>
                        </div>
                        <div class="overlay"></div>
                </>
           }
        }}
    </div>
    };
}

async fn send_request_to_api(request: LandingPageRequest) -> String {
    let landing_page_api = "https://landing-page-rs-backend.onrender.com/api/landing_page";
    
    let client = reqwest::Client::new();
    let mut body_map =  HashMap::new();
    body_map.insert("name", request.name);
    body_map.insert("telephone_number", request.telephone_number);
    body_map.insert("email", request.email);
    body_map.insert("already_have_the_product", request.already_have_the_product.to_string());
    body_map.insert("want_to_receive_more_info", request.want_to_receive_more_info.to_string());
    

    match client
        .post(landing_page_api)
        .json(&body_map)
        .send()
        .await {
            Ok(response) => {
                match response.json::<LandingPageResponse>().await {
                    Ok(parsed_response) => {
                        if parsed_response.status {
                            "Requisição bem-sucedida".to_string()
                        } else {
                            "Requisição Falhou".to_string()
                        }
                    },

                    Err(_) => {
                        "Requisição Falhou".to_string()
                    }
                }
            },
            Err(_) => {
                "Requisição Falhou".to_string()
            }
        }
    }
