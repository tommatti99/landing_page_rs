use yew::prelude::*;


pub struct ResultBox {    
    pub on: bool,
    pub text: String
}       

enum Msg {
    HideResultBox   
}

impl Component for ResultBox {
    
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            on = false,
            text = "".to_string()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {

            Msg::HideResultBox => {
                self.on = false;
                self.text = "".to_string();
            }
        }
        
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let hide = {
            Callback::from(move |_: MouseEvent {Msg::HideResultBox});
        };
        
        html! {
            <>
                <div class ="result_box">
                    <h2>{ (self.text).clone() }</h2>
                    <button onclick = {toggle} style={format!("padding: 0.5em 1em; background-color: white; color: black; border: none; border-radius: 4px; cursor: pointer; font-size: 1em;")}>
                        {"Ok!"}
                    </button>
                </div>
                <div class="overlay"></div>
            </>
        }
    }
}