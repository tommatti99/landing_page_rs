use yew::prelude::*;


#[derive(Properties, Part_Eq, Eq, Clone)]
struct ResultBoxProps {
    on: bool,
    text: String
}

struct ResultBox {    
    on: bool,
    text: String
}       

enum Msg {
    HideResultBox   
}

impl Component for ResultBox {
    
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            on: ctx.on,
            text: ctx.text
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {

            Msg::HideResultBox => {
                self.text = "".to_string();
                self.on = false;
            }
        }
        
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let hide = {
            Callback::from(move |_: MouseEvent {Msg::HideResultBox})
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