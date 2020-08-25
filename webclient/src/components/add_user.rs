use yew::{html, Callback, Component, ComponentLink, Html, ShouldRender, Properties, InputData};
use crate::widget;

pub struct AddUser {
    user_name: String,
    onsaved: Callback<String>,
    oncancel: Callback<()>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Cancel,
    InputUserName(String),
    Save,
}

#[derive(Clone, PartialEq, Properties)]
pub struct AddUserProps {
    pub onsaved: Callback<String>,
    pub oncancel: Callback<()>,
}

impl Component for AddUser {
    type Message = Msg;
    type Properties = AddUserProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        AddUser {
            user_name: String::new(),
            onsaved: props.onsaved,
            oncancel: props.oncancel,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Cancel => {
                self.oncancel.emit(());
                false
            },
            Msg::InputUserName(user_name) => {
                let user_name = user_name.trim();
                if !user_name.is_empty() {
                    self.user_name = user_name.to_string();
                }
                false
            },
            Msg::Save => {
                if self.user_name.is_empty() {
                    true
                } else {
                    self.onsaved.emit(self.user_name.clone());
                    false
                }
            },
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div class = "adduser-input">
                    { widget::input("input", "User Name", Some(self.link.callback(|input_data: InputData| Msg::InputUserName(input_data.value)))) }
                </div>
                <div class = "left-button">
                    { widget::button("button", "Cancel", self.link.callback(|_| Msg::Cancel)) }
                </div>
                <div class = "right-button">
                    { widget::button("button", "Save", self.link.callback(|_| Msg::Save)) }
                </div>
            </>
        }
    }
}