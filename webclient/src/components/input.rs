use yew::{html, Callback, MouseEvent, Component, ComponentLink, Html, ShouldRender, Properties};

pub struct Input {
    id: String,
    text: String,
    value: String,
    clicked: bool,
    onclick: Callback<MouseEvent>,
    oninput: Callback<MouseEvent>,
}

pub enum Msg {
    Click,
    GotInput(String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct InputProps {
    pub id: String,
    pub text: String,
    pub oninput: Callback<MouseEvent>,
}

impl Component for Input {
    type Message = Msg;
    type Properties = InputProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Input {
            id: props.id,
            text: props.text,
            value: String::new(),
            clicked: false,
            onclick: link.callback(|_| Msg::Click),
            oninput: link.callback(|_| Msg::GotInput("".to_string())),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = true;
                //true // Indicate that the Component should re-render
            },
            Msg::GotInput(new_value) => {
                self.value = new_value;
            },
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.id = props.id;
        self.text = props.text;
        true
    }


}