use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
}

#[function_component(Home)]
fn home() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <main class="h-screen bg-black flex flex-col items-center justify-center">
            <p class="text-8xl mb-10 text-white">{ *counter }</p>
            <button class="bg-white text-black w-3/12 mx-auto py-3 rounded-xl" {onclick}>{ "+1" }</button>
        </main>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home />},
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
