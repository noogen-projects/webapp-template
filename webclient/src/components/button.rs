use yew::{html, Callback, MouseEvent, Component, ComponentLink, Html, ShouldRender, Properties};

pub struct Button {
    id: String,
    text: String,
    onclick: Callback<MouseEvent>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    pub id: String,
    pub text: String,
    pub onclick: Callback<MouseEvent>,
}

impl Component for Button {
    type Message = ();
    type Properties = ButtonProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Button {
            id: props.id,
            text: props.text,
            onclick: props.onclick,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.id = props.id;
        self.text = props.text;
        self.onclick = props.onclick;
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