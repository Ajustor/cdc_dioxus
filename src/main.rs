use dioxus::prelude::*;

mod components;
mod guide_router;
mod layouts;
mod pages;

use guide_router::Routes;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        Router::<Routes> {}
    }
}
