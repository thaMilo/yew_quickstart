use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
use pages::home::home::Home;
use pages::login::login::Login;
use pages::register::register::Register;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register
}
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home />},
        Route::Login => html! {<Login />},
        Route::Register => html! {<Register />}
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch}/>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
