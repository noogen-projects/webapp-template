use yew::{html, Callback, MouseEvent, Component, ComponentLink, Html, ShouldRender};

struct Root {
    clicked: bool,
    onclick: Callback<MouseEvent>,
}

enum Msg {
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
            <button onclick = &self.onclick>{ button_text }</button>
        }
    }
}

fn main() {
    yew::start_app::<Root>();
}