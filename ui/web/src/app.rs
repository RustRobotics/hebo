// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::components::about::AboutComponent;
use crate::components::connections::ConnectionsComponent;
use crate::components::log::LogComponent;
use crate::components::new_connection::NewConnectionComponent;
use crate::components::settings::SettingsComponent;
use yew::prelude::{html, Component, Context, Html};
use yew_router::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[at("/connections")]
    Connections,

    #[at("/new")]
    NewConnection,

    #[at("/log")]
    Log,

    #[at("/about")]
    About,

    #[at("/settings")]
    Settings,

    #[at("/")]
    Home,
}

pub struct AppModel {}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <ConnectionsComponent/> },
        Route::Connections => html! { <ConnectionsComponent/> },
        Route::NewConnection => html! { <NewConnectionComponent/> },
        Route::Log => html! { <LogComponent/> },
        Route::About => html! { <AboutComponent/> },
        Route::Settings => html! { <SettingsComponent/> },
    }
}

impl Component for AppModel {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        }
    }
}
