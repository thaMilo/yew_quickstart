use yew::prelude::*;
use yew::{classes, html};

#[function_component(Register)]
pub fn register() -> Html {
    html! {
        <main class={classes!(String::from("h-screen bg-foreground relative"))}>
            <img src="./assets/images/logo.svg" class={classes!(String::from("absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2"))}/>
        </main>
    }
}
