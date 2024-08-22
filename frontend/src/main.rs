use yew_router::prelude::*;
use yew::prelude::*;
mod pages;
use pages::home::Home;
use pages::kryds::Krydsbole;



#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/krydsbole")]
    Krydsbole,
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
        Route::Krydsbole => html! { <Krydsbole /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Nav />
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </>
    }
}

#[function_component(Nav)]
fn nav() -> Html {
    return html! {
        <nav class={classes!("navbar")}>
            <ul class={classes!("navbar-menu")}>
                <li class={classes!("navbar-item")}><a href="/">{"Hjem"}</a></li>
                <li class={classes!("navbar-item")}><a href="/krydsbole">{"super kryds og bole"}</a></li>
                <li class={classes!("navbar-item")}><a href="/about">{"om"}</a></li>
            </ul>
        </nav>
    };
}
