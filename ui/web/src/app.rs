// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::prelude::{html, Component, Context, Html};
use yew_router::prelude::*;

use crate::components::about::AboutComponent;
use crate::components::connections::ConnectionsComponent;
use crate::components::log::LogComponent;
use crate::components::new_connection::NewConnectionComponent;
use crate::components::settings::SettingsComponent;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/connections")]
    Connections,

    #[at("/new")]
    NewConnection,

    #[at("/logs")]
    Logs,

    #[at("/about")]
    About,

    #[at("/settings")]
    Settings,

    #[at("/")]
    Home,
}

pub struct AppComponent {}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <ConnectionsComponent/> },
        Route::Connections => html! { <ConnectionsComponent/> },
        Route::NewConnection => html! { <NewConnectionComponent/> },
        Route::Logs => html! { <LogComponent/> },
        Route::About => html! { <AboutComponent/> },
        Route::Settings => html! { <SettingsComponent/> },
    }
}

impl AppComponent {
    fn left_panel(&self) -> Html {
        html! {
            <nav class="left-panel" role="navigation" aria-label="main navigation">
                <ul>
                    <li><Link<Route> to={Route::Home}>{"Home"}</Link<Route>></li>
                    <li><Link<Route> to={Route::NewConnection}>{"New"}</Link<Route>></li>
                    <li><Link<Route> to={Route::Logs}>{"Logs"}</Link<Route>></li>

                    <li><Link<Route> to={Route::About}>{"About"}</Link<Route>></li>
                    <li><Link<Route> to={Route::Settings}>{"Settings"}</Link<Route>></li>
                </ul>
            </nav>
        }
    }
}

impl Component for AppComponent {
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
            <>
                <BrowserRouter>
                    {self.left_panel() }

                    <div class="main-content">
                        <Switch<Route> render={Switch::render(switch)} />
                    </div>
                </BrowserRouter>
            </>
        }
    }
}
