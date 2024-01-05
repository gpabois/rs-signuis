use std::collections::HashMap;

use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::Routable;
use yew_router::history::{AnyHistory, History, MemoryHistory};

mod pages;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => {
            html! { <pages::Home/> }
        },
        Route::NotFound => {
            html! { <pages::NotFound/> }
        }
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <main>
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
    }
}

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
}

#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Router history={history}>
            <main>
                <Switch<Route> render={switch} />
            </main>
        </Router>
    }
}