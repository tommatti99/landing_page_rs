use std::collections::HashMap;
use yew::prelude::*;
use reqwest;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use crate::components::result_box::ResultBox;


struct LandingPageRequest {
    name: String,
    surname: String,
    prefix_telephone_number: String,
    telephone_number: String, 
    cpf: String,
    email: String,
    birth: String,
    address: String,
    city: String,
    state: String,
    terms_and_cond: bool
} 
#[derive(Deserialize, Debug, Clone)]
struct LandingPageResponse {
    success: bool
}

struct ResBoxState {
    on: bool,
    text: String
}

#[function_component]
pub fn Form() -> Html {
    let name = use_state(|| String::new());
    let surname = use_state(|| String::new());
    let email = use_state(|| String::new());
    let prefix_telephone_number = use_state(|| String::new());
    let telephone_number = use_state(|| String::new());
    let cpf = use_state(|| String::new());
    let birth = use_state(|| String::new());
    let address = use_state(|| String::new());
    let city = use_state(|| String::new());
    let state = use_state(|| String::new());
    let terms_and_cond = use_state(|| false); 
    
    let res_box = use_state(|| ResBoxState {on: false, text: "".to_string()}); 

    let name_input = {
        let name = name.clone();

        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                name.set(input.value());
        }
        })
    };

    let surname_input = {
        let surname = surname.clone();

        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                surname.set(input.value());
        }
        })
    };

    let prefix_telephone_number_input = {
        let prefix_telephone_number = prefix_telephone_number.clone();
        
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                prefix_telephone_number.set(input.value());
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

    let cpf_input = {
        let cpf = cpf.clone();
        
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                cpf.set(input.value());
        }
        })
    };

    let birth_input = {
        let birth = birth.clone();
        
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                birth.set(input.value());
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

    let address_input = {
        let address = address.clone();
    
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                address.set(input.value());
        }
        })
    };

    let state_input = {
        let state = state.clone();
    
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                state.set(input.value());
        }
        })
    };
    
    let city_input = {
        let city = city.clone();
    
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                city.set(input.value());
        }
        })
    };
    
    
    let terms_and_cond_input = {
        let terms_and_cond = terms_and_cond.clone();
    
        Callback::from(move |e: MouseEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                terms_and_cond.set(input.checked());
        }
        })
    };


    let click_submit = {
        let state_clone_name = name.clone();
        let state_clone_surname = surname.clone();
        let state_clone_prefix_telephone_number = prefix_telephone_number.clone();
        let state_clone_telephone_number = telephone_number.clone();
        let state_clone_cpf = cpf.clone();
        let state_clone_email = email.clone();
        let state_clone_birth = birth.clone();
        let state_clone_address = address.clone();
        let state_clone_city = city.clone();
        let state_clone_state = state.clone();
        let state_clone_terms_and_cond = terms_and_cond.clone();

        let res_box = res_box.clone(); 
    

        Callback::from(move |_: MouseEvent| {
            let request: LandingPageRequest = LandingPageRequest {
                name: (*state_clone_name).clone(),
                surname: (*state_clone_surname).clone(),
                prefix_telephone_number: (*state_clone_prefix_telephone_number).clone(),
                telephone_number: (*state_clone_telephone_number).clone(),
                cpf: (*state_clone_cpf).clone(),
                email: (*state_clone_email).clone(),
                birth: (*state_clone_birth).clone(),
                address: (*state_clone_address).clone(),
                city: (*state_clone_city).clone(),
                state: (*state_clone_state).clone(),
                terms_and_cond: (*state_clone_terms_and_cond).clone()
            };

            let res_box = res_box.clone(); 
            
            spawn_local(async move {
                let msg = send_request_to_api(request).await;
                res_box.set(ResBoxState {on: true, text: msg});
            });
            
        })
    };

    return html!{
        <div style={format!("display: flex; justify-content: center; align-items: center; height: 100vh; color: white;")}>
            <div style={format!("background-color: rgba(102, 102, 102, 0.1); backdrop-filter: blur(8px); width:53rem; border-radius: 1rem")}>
            <div style={format!("flex-direction: column; align-items: center; display: flex; width: 50rem; padding: 1em; border-radius: 8px;")}>
                <h1>{"Cadastro Afiliados Fusan"}</h1>
                <div style={format!("display: flex; gap: 3rem; flex-direction: row; width:100%;")}>
                    <div style={format!("width: 50%; margin-bottom: 1em;")}>
                        <h2 style={format!("margin: 0.5em 0; font-size: 1em;")}>{"Nome"}</h2>
                        <input type="text" id="name" value={(*name).clone()} oninput={name_input} style={format!("width: 100%; padding: 0.5em; border-radius: 4px; border: 1px solid #ccc;")} />
                    </div>
                    <div style={format!("width: 50%; margin-bottom: 1em;")}>
                        <h2 style={format!("margin: 0.5em 0; font-size: 1em;")}>{"Sobrenome"}</h2>
                        <input type="text" id="name" value={(*surname).clone()} oninput={surname_input} style={format!("width: 100%; padding: 0.5em; border-radius: 4px; border: 1px solid #ccc;")} />
                    </div>
                </div>

                <div style={format!("display: flex; gap: 3rem; flex-direction: row; width:100%;")}>
                    <div style={format!("width: 7%; margin-bottom: 1em;")}>
                        <h2 style={format!("margin: 0.5em 0; font-size: 1em;")}>{"DDD"}</h2>
                        <input type="text" id="telephone_number" value={(*prefix_telephone_number).clone()} oninput={prefix_telephone_number_input} style={format!("width: 100%; padding: 0.5em; border-radius: 4px; border: 1px solid #ccc;")} />
                    </div>
                    <div style={format!("width: 40%; margin-bottom: 1em;")}>
                        <h2 style={format!("margin: 0.5em 0; font-size: 1em;")}>{"Telefone"}</h2>
                        <input type="text" id="telephone_number" value={(*telephone_number).clone()} oninput={telephone_number_input} style={format!("width: 100%; padding: 0.5em; border-radius: 4px; border: 1px solid #ccc;")} />
                    </div>
                    <div style={format!("width: 53%; margin-bottom: 1em;")}>
                        <h2 style={format!("margin: 0.5em 0; font-size: 1em;")}>{"CPF"}</h2>
                        <input type="text" id="telephone_number" value={(*cpf).clone()} oninput={cpf_input} style={format!("width: 100%; padding: 0.5em; border-radius: 4px; border: 1px solid #ccc;")} />
                    </div>
                </div>

                <div style={format!("display: flex; gap: 3rem; flex-direction: row; width:100%;")}>
                    <div style={format!("width: 70%; margin-bottom: 1em;")}>
                        <h2 style={format!("margin: 0.5em 0; font-size: 1em;")}>{"Email"}</h2>
                        <input type="text" id="email" value={(*email).clone()} oninput={email_input} style={format!("width: 100%; padding: 0.5em; border-radius: 4px; border: 1px solid #ccc;")} />
                    </div>
                    <div style={format!("width: 30%; margin-bottom: 1em;")}>
                        <h2 style={format!("margin: 0.5em 0; font-size: 1em;")}>{"Data de Nascimento"}</h2>
                        <input type="date" id="telephone_number" value={(*birth).clone()} oninput={birth_input} style={format!("width: 100%; padding: 0.5em; border-radius: 4px; border: 1px solid #ccc;")} />
                    </div>
                </div>

                <div style={format!("display: flex; gap: 3rem; flex-direction: row; width:100%;")}>
                    <div style={format!("width: 76%; margin-bottom: 1em;")}>
                        <h2 style={format!("margin: 0.5em 0; font-size: 1em;")}>{"Endereço"}</h2>
                        <input type="text" id="address" value={(*address).clone()} oninput={address_input} style={format!("width: 100%; padding: 0.5em; border-radius: 4px; border: 1px solid #ccc;")} />
                    </div>
                    <div style={format!("width: 14%; margin-bottom: 1em;")}>
                        <h2 style={format!("margin: 0.5em 0; font-size: 1em;")}>{"Cidade"}</h2>
                        <input type="text" id="address" value={(*city).clone()} oninput={city_input} style={format!("width: 100%; padding: 0.5em; border-radius: 4px; border: 1px solid #ccc;")} />
                    </div>
                    <div style={format!("width: 10%; margin-bottom: 1em;")}>
                        <h2 style={format!("margin: 0.5em 0; font-size: 1em;")}>{"Estado"}</h2>
                        <input type="text" id="address" value={(*state).clone()} oninput={state_input} style={format!("width: 100%; padding: 0.5em; border-radius: 4px; border: 1px solid #ccc;")} />
                    </div>
                </div>
                <div style={format!("width: 100%; margin-bottom: 1em;")}>
                    <h2 style={format!("margin: 0.5em 0; font-size: 1em;")}><input type="checkbox" id="terms_and_cond" checked={*terms_and_cond} onclick={terms_and_cond_input}  style={format!("transform: scale(1.5);")} />{"  Concordo com os termos e condições"}</h2>
                
                </div>
                <button onclick = {click_submit} style={format!("padding: 0.5em 1em; background-color: white; color: black; border: none; border-radius: 4px; cursor: pointer; font-size: 1em;")}>
                {"Cadastrar"}
                </button>
            </div>
        </div>
        {   if res_box.on {
                html!{
                    <ResultBox text={(*res_box).text.clone()} />
                }
            } else {

                html!{
                    <div></div>
                }
            }
        }
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
    body_map.insert("address", request.address.to_string());
    body_map.insert("terms_and_cond", request.terms_and_cond.to_string());
    

    let response = client
        .post(landing_page_api)
        .json(&body_map)
        .send()
        .await;

    match response {
        Ok(res) => {
            let deserealized_res: LandingPageResponse = serde_json::from_str(&res.text().await.unwrap()).unwrap();
                if deserealized_res.success {
                    "Requisição bem-sucedida".to_string()
                } else {
                    "Requisição Falhou".to_string()
                }
        },
        Err(e) => {
            format!("Requisição Falhou: {}", e)
        }
    }
}
