use yew_router::prelude::*;
use yew::prelude::*;
mod pages;
mod comp;
mod model;
mod func;
use pages::home::Home;
use pages::auth::{login::Login, register::Register};
use pages::kryds::{Menu, local::Krydsbole as Local, online::Krydsbole as Online,bot::Krydsbole as Bot};


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/krydsbole")]
    Krydsbole,
    #[at("/krydsbole/local")]
    Local,
    #[at("/krydsbole/online")]
    Online,
    #[at("/krydsbole/bot")]
    Bot,
    #[not_found]
    #[at("/404")]
    NotFound,
}


fn main() {
    yew::Renderer::<App>::new().render();
}


fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Register=> html! { <Register /> },
        Route::Login => html! { <Login /> },
        Route::Krydsbole => html! { <Menu /> },
        Route::Local => html! { <Local /> },
        Route::Online => html! { <Online /> },
        Route::Bot=> html! { <Bot /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
            <BrowserRouter>
                <Nav />
                <Switch<Route> render={switch} />
            </BrowserRouter>
    }
}

#[function_component(Nav)]
fn nav() -> Html {
    return html! {
        <nav class={classes!("navbar")}>
            <ul class={classes!("navbar-menu")}>
                <li class={classes!("navbar-item")}><Link<Route> to={Route::Home}>{ "Hjem" }</Link<Route>></li>
                <li class={classes!("navbar-item")}><Link<Route> to={Route::Login}>{ "Login" }</Link<Route>></li>
                <li class={classes!("navbar-item")}><Link<Route> to={Route::Register}>{ "Register" }</Link<Route>></li>
                <li class={classes!("navbar-item")}><Link<Route> to={Route::Krydsbole}>{ "super kryds og bole" }</Link<Route>></li>
            </ul>
        </nav>
    };
}
