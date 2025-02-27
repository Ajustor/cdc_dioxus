use dioxus::prelude::*;

use crate::guide_router::Routes;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

#[component]
pub fn IndexLayout() -> Element {
    // Build cool things ✌️
    let route = use_route::<Routes>();

    rsx! {
      // Global app resources
      document::Link { rel: "icon", href: FAVICON }
      document::Link { rel: "stylesheet", href: MAIN_CSS }

      nav {
        if route == Routes::Rules {
          Link { to: Routes::Index, "<-" }
        } else {
          Link { to: Routes::Rules, "Règles" }
        }
      }

      Outlet::<Routes> {}
    }
}
