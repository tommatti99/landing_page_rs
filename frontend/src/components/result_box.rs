use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ResultBoxProps {
    pub text: String
}

pub struct ResultBox {    
    pub on: bool
}       

pub enum Msg {
    HideResultBox   
}

impl Component for ResultBox {
    
    type Message = Msg;
    type Properties = ResultBoxProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            on: true
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HideResultBox => {
                self.on = false;
                web_sys::window().unwrap().location().reload().unwrap();
            }
        }
        
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let hide = ctx.link().callback(|_: MouseEvent| Msg::HideResultBox);
        
        html! {
            <>
                { 
                if self.on {
                    html! {
                        <div>
                            <div class="result_box">
                                <h2>{ (&ctx.props().text).clone() }</h2>
                                <button onclick = {hide} style={format!("width: 5em; height: 2em; display: flex; outline: none; align-content: center; align-items: center; justify-content: center; background-color: white; color: black; border: 1px, solid, black; border-radius: 4px; cursor: pointer; font-size: 1em;")}>
                                    {"Ok!"}
                                </button>
                            </div>
                            <div class="overlay"></div>
                        </div>
                }} else {
                    html! {<div></div>}
                    }
                }
            </>
        }
    }
}