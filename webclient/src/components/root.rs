use yew::{html, Component, ComponentLink, Html, ShouldRender};
use super::{UserTable};
use crate::{dto::User, widget, components::AddUser};

pub struct Root {
    link: ComponentLink<Self>,
    users: Vec<User>,
    state: State,
}

pub enum Msg {
    ClickShowAddUser,
    ClickAddUser,
    ClickShowUser,
    Back,
    SavedUser(String),
    Cancel,
}

pub enum State {
    ShowUsersList,
    ShowUser,
    ChangeUser,
    AddUser,
}

impl Component for Root {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Root {
            link,
            users: vec![User::new(1,
            "Vasia"), User::new(2, "Anna"), User::new(3, "Peter")],
            state: State::ShowUsersList,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ClickShowAddUser => {
                self.state = State::AddUser;
            },
            Msg::ClickAddUser => {
                self.state = State::AddUser;
            },
            Msg::ClickShowUser => {
                self.state = State::ShowUser;
            },
            Msg::Back => {
                self.state = State::ShowUsersList;
            },
            Msg::SavedUser(user_name) => {
                let user_id = self.users.len() + 1;
                self.users.push(User::new(user_id, user_name));
                self.state = State::ShowUsersList;
            },
            Msg::Cancel => (),
        }
        true
    }

    fn view(&self) -> Html {
        match self.state {
            State::ShowUsersList => {
                self.view_users_list()
            },
            State::ShowUser => {
                self.view_user_was_added()
            },
            State::AddUser => {
                self.view_add_user()
            },
            State::ChangeUser => {
                self.view_change_user()
            },
        }
    }
}

impl Root {
    fn view_users_list(&self) -> Html {
        let users = self.users.clone();
        html! {
            <>
                { widget::header() }
                <div class = "mdc-top-app-bar--fixed-adjust">
                    <div class = "content">
                        <h1 class = "mdc-typography--headline4">{ "Users List" }</h1>
                        <UserTable id = "user_table" caption = "Users" users = users />
                        <div class = "right-button">
                            { widget::button("button", "Add user", self.link.callback(|_| Msg::ClickAddUser)) }
                        </div>
                    </div>
                </div>
            </>
        }
    }

    fn view_user(&self, user: &User) -> Html {
        let users = vec![user.clone()];
        html! {
            <>
                { widget::header() }
                <div class = "mdc-top-app-bar--fixed-adjust">
                    <div class = "content">
                        <h1 class = "mdc-typography--headline4">{ "User was added" }</h1>
                        <UserTable id = "user_table" caption = "User was added" users = users />
                        <div class = "adduser-button">
                            { widget::button("button", "Back", self.link.callback(|_| Msg::Back)) }
                        </div>
                    </div>
                </div>
            </>
        }
    }

    fn view_add_user(&self) -> Html {
        html! {
            <>
                { widget::header() }
                <div class = "mdc-top-app-bar--fixed-adjust">
                    <div class = "content">
                        <h1 class = "mdc-typography--headline4">{ "Add new User" }</h1>
                        <AddUser onsaved = self.link.callback(|user_name| Msg::SavedUser(user_name)) oncancel = self.link.callback(|_| Msg::Cancel)/>
                    </div>
                </div>
            </>
        }
    }

    fn view_user_was_added(&self) -> Html {
        let last_user = self.users.last();
        match last_user {
            Some(user) => self.view_user(user),
            None =>
                html! {
                    <>
                        { widget::header() }
                        <div class = "mdc-top-app-bar--fixed-adjust">
                            <div class = "content">
                                <h1 class = "mdc-typography--headline4">{ "Add New User" }</h1>
                                <p>{"User was not found"}</p>
                                <div class = "adduser-button">
                                    { widget::button("button", "Back", self.link.callback(|_| Msg::Back)) }
                                </div>
                            </div>
                        </div>
                    </>
                }
        }
    }

    fn view_change_user(&self) -> Html {
        html! {
            <></>
        }
    }
}