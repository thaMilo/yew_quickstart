use yew::prelude::*;
use yew::{classes, html};

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <main class={classes!(String::from("h-screen bg-foreground relative"))}>
            <div class={classes!(String::from("absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 flex flex-col items-center justify-center"))}>
                <h1 class={classes!(String::from("font-primary text-2xl font-bold text-foreground_dark"))}>{"Login"}</h1>
            </div>
        </main>
    }
}
