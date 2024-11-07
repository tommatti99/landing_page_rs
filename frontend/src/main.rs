use yew::prelude::*;
use crate::components::{footer::Footer, form::Form, header::Header};

pub mod components {
    pub mod footer;
    pub mod form;
    pub mod header;
}


#[function_component(Main)]
pub fn app() -> Html {
    return html! {
        <div style = {format!("")}>
            <Header/>
            <Form/>
            <Footer/>
        </div>
    }
}


pub fn main() {
    yew::Renderer::<Main>::new().render();
}