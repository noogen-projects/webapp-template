use yew::{html, Callback, MouseEvent, Component, ComponentLink, Html, ShouldRender, Properties};
use crate::dto::User;

pub struct UserTable {
    id: String,
    caption: String,
    users: Vec<User>,
    clicked: bool,
    onclick: Callback<MouseEvent>,
}

pub enum Msg {
    Click,
}

#[derive(Clone, PartialEq, Properties)]
pub struct UserTableProps {
    pub id: String,
    pub caption: String,
    pub users: Vec<User>,
}

impl Component for UserTable {
    type Message = Msg;
    type Properties = UserTableProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        UserTable {
            id: props.id,
            caption: props.caption,
            users: props.users,
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
        self.caption = props.caption;
        self.users = props.users;
        true
    }

    fn view(&self) -> Html {
        //let mdc_init = format!("mdc.ripple.MDCRipple.attachTo(document.getElementById('{}'))", self.id);
        let row_number = self.users.len();
        html! {
          <div class = "user-table mdc-data-table">
              <table id = self.id class = "mdc-data-table__table" aria-label = self.caption>
                <thead>
                  <tr class = "mdc-data-table__header-row">
                    <th class = "mdc-data-table__header-cell mdc-data-table__header-cell--numeric" role = "columnheader" scope = "col">{ "User Id" }</th>
                    <th class = "mdc-data-table__header-cell" role = "columnheader" scope = "col">{ "User Name" }</th>
                  </tr>
                </thead>
                <tbody class = "mdc-data-table__content">
                   { (0..row_number).map(|row| self.view_row(&self.users[row])).collect::<Html>() }
                </tbody>
              </table>
           </div>
        }
    }
}

impl UserTable {
    fn view_row(&self, user: &User) -> Html {
        html! {
            <tr class = "mdc-data-table__row">
                <td class = "mdc-data-table__cell mdc-data-table__cell--numeric">{ &user.id }</td>
                <td class = "mdc-data-table__cell">{ &user.name }</td>
            </tr>
        }
    }
}