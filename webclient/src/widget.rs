use yew::{html, Callback, MouseEvent, InputData, Html};

pub fn header() -> Html {
    html! {
        <header class = "mdc-top-app-bar mdc-top-app-bar--fixed">
            <div class = "mdc-top-app-bar__row">
                <section class = "mdc-top-app-bar__section mdc-top-app-bar__section--align-start">
                <button class = "mdc-icon-button material-icons mdc-top-app-bar__navigation-icon--unbounded">{ "menu" }</button>
                <span class = "mdc-top-app-bar__title">{ "Webapp Template" }</span>
                </section>
            </div>
        </header>
    }
}

pub fn button(id: impl AsRef<str>, text: impl AsRef<str>, onclick: Callback<MouseEvent>) -> Html {
    let id = id.as_ref();
    let text = text.as_ref();
    let mdc_init = format!("mdc.ripple.MDCRipple.attachTo(document.getElementById('{}'))", id);
    html! {
        <button id = id class = "mdc-button" onclick = onclick>
            <span class = "mdc-button__ripple"></span>
            { text }
            <script>{ mdc_init }</script>
        </button>
    }
}

pub fn input(id: impl AsRef<str>, text: impl AsRef<str>, oninput: Option<Callback<InputData>>) -> Html {
    let id = id.as_ref();
    let text = text.as_ref();
    let mdc_init = format!("mdc.textField.MDCTextField.attachTo(document.getElementById('{}'))", id);
    let input_id = format!("{}-input", id);

    let input = if let Some(callback) = oninput {
        html! {
            <input id = &input_id class = "mdc-text-field__input" oninput = callback />
        }
    } else {
        html! {
            <input id = &input_id class = "mdc-text-field__input" />
        }
    };

    html! {
        <div id = id class = "mdc-text-field mdc-text-field--outlined">
          { input }
          <div class = "mdc-notched-outline">
            <div class = "mdc-notched-outline__leading"></div>
            <div class = "mdc-notched-outline__notch">
              <label for = &input_id class = "mdc-floating-label">{ text }</label>
            </div>
            <div class="mdc-notched-outline__trailing"></div>
          </div>
          <script>{ mdc_init }</script>
        </div>
    }
}