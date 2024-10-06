mod pages;
mod comp;
mod model;
mod func;
mod state;

use yew_router::prelude::{Routable,BrowserRouter,Switch,Link};
use yew::prelude::{Html,html,function_component,classes};
use pages::home::Home;
use pages::kryds::{Menu, Local, Online, Bot};
use pages::auth::login::Login;
// use pages::auth::register::Register;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
 //   #[at("/register")}
 //   Register,
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


fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
//        Route::Register=> html! { <Register /> },
        Route::Login => html! { <Login /> },
        Route::Krydsbole => html! { <Menu /> },
        Route::Local => html! { <Local /> },
        Route::Online => html! { <Online /> },
        Route::Bot=> html! { <Bot /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
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
                <li class={classes!("navbar-item")}><Link<Route> to={Route::Krydsbole}>{ "super kryds og bole" }</Link<Route>></li>
            </ul>
        </nav>
    };
}
