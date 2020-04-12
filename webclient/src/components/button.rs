use yew::{html, Callback, MouseEvent, Component, ComponentLink, Html, ShouldRender, Properties};

pub struct Button {
    id: String,
    text: String,
    clicked: bool,
    onclick: Callback<MouseEvent>,
}

pub enum Msg {
    Click,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    pub id: String,
    pub text: String,
}

impl Component for Button {
    type Message = Msg;
    type Properties = ButtonProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Button {
            id: props.id,
            text: props.text,
            clicked: false,
            onclick: link.callback(|_| Msg::Click),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = true;
                true // Indicate that the Component should re-render
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.id = props.id;
        self.text = props.text;
        true
    }

    fn view(&self) -> Html {
        let mdc_init = format!("mdc.ripple.MDCRipple.attachTo(document.getElementById('{}'))", self.id);
        html! {
            <button id = self.id class = "mdc-button" onclick = &self.onclick>
                <span class = "mdc-button__ripple"></span>
                { &self.text }
                <script>{ mdc_init }</script>
            </button>
        }
    }
}