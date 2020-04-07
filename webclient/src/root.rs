use yew::{html, Callback, MouseEvent, Component, ComponentLink, Html, ShouldRender};

pub struct Root {
    clicked: bool,
    onclick: Callback<MouseEvent>,
}

pub enum Msg {
    Click,
}

impl Component for Root {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Root {
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

    fn view(&self) -> Html {
        let button_text = if self.clicked { "Clicked!" } else { "Click me!" };
        html! {
            <button id = "button" class = "mdc-button" onclick = &self.onclick>
                <span class = "mdc-button__ripple"></span>
                { button_text }
                <script>{ "mdc.ripple.MDCRipple.attachTo(document.getElementById('button'))" }</script>
            </button>
        }
    }
}