use yew::{html, Component, ComponentLink, Html, ShouldRender};
use super::{Button, UserTable};
use crate::dto::User;

pub struct Root;

pub enum Msg {
    Click,
}

impl Component for Root {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                true // Indicate that the Component should re-render
            }
        }
    }

    fn view(&self) -> Html {
        let users = vec![User::new(1, "Vasia"), User::new(2, "Anna"), User::new(3, "Peter")];
        html! {
            <>
                <header class = "mdc-top-app-bar mdc-top-app-bar--fixed">
                    <div class = "mdc-top-app-bar__row">
                        <section class = "mdc-top-app-bar__section mdc-top-app-bar__section--align-start">
                            <button class = "mdc-icon-button material-icons mdc-top-app-bar__navigation-icon--unbounded">{ "menu" }</button>
                            <span class = "mdc-top-app-bar__title">{ "Webapp Template" }</span>
                        </section>
                    </div>
                </header>
                <div class = "mdc-top-app-bar--fixed-adjust">
                    <div class = "content">
                        <h1 class = "mdc-typography--headline4">{ "Users List" }</h1>
                        <UserTable id = "user_table" caption = "Users" users = users />
                        <div class = "adduser-button">
                            <Button id = "button" text = "Add user" />
                        </div>
                    </div>
                </div>
             </>
        }
    }
}