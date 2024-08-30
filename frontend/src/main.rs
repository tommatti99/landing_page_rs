use yew::prelude::*;
use crate::components::{footer::Footer, form::Form, header::Header};



#[function_component(Main)]
pub fn app() -> Html {
    return html! {
        <div>
            <Header/>
            <Form/>
            <Header/>
        </div>
    }
}


pub fn main() {
    yew::Renderer::<Main>::new().render();
}