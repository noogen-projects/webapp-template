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
            <button class = "mdc-button" onclick = &self.onclick>
                <span class = "mdc-button__ripple"></span>
                { button_text }
            </button>
        }
    }
}

fn main() {
    yew::initialize();
    yew::App::<Root>::new().mount_to_body();
    js_sys::eval(r#"
        /* Initialize MDC Web components. */
        const buttons = document.querySelectorAll(".mdc-button");
        for (const button of buttons) {
            mdc.ripple.MDCRipple.attachTo(button);
        }

        const textFields = document.querySelectorAll(".mdc-text-field");
        for (const textField of textFields) {
            mdc.textField.MDCTextField.attachTo(textField);
        }
    "#).expect("Cannot eval MDC Web components initialization code");
    yew::run_loop();
}