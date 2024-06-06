use yew::prelude::*;
use yew::{classes, html};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <main class={classes!(String::from("h-screen bg-foreground relative"))}>
            <div class={classes!(String::from("absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 flex flex-col items-center justify-center"))}>
                <div class={classes!(String::from("flex items-center justify-center"))} >
                    <img src="images/yew.svg" class={classes!(String::from("w-52 h-52 mb-7 -rotate-[20deg] -mr-10 z-[100]"))}/>
                    <img src="images/tailwind.svg" class={classes!(String::from("w-52 h-52 mb-7 rotate-[20deg] -ml-10"))}/>
                </div>
                <h1 class={classes!(String::from("font-primary text-2xl font-bold text-foreground_dark"))}>{"Welcome to the Yew quickstart"}</h1>
            </div>
        </main>
    }
}
