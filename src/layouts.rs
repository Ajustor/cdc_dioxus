use dioxus::prelude::*;

use crate::guide_router::Routes;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

#[derive(serde::Deserialize, Debug)]
struct Citation {
    pub citation: String,
}

#[derive(serde::Deserialize, Debug)]
struct KaamelotApi {
    pub citation: Citation,
}

#[component]
pub fn IndexLayout() -> Element {
    // Build cool things ✌️
    let route = use_route::<Routes>();
    let mut quote = use_signal(|| "".to_string());

    let fetch_new_quote = move |_| async move {
        let response = reqwest::get("https://kaamelott.chaudie.re/api/random/personnage/Perceval")
            .await
            .unwrap()
            .json::<KaamelotApi>()
            .await
            .unwrap();

        quote.set(response.citation.citation);
    };

    rsx! {
      // Global app resources
      document::Link { rel: "icon", href: FAVICON }
      document::Link { rel: "stylesheet", href: MAIN_CSS }

      nav { id: "header",
        if route == Routes::Rules {
          Link { to: Routes::Index, "<-" }
        } else {
          Link { to: Routes::Rules, "Règles" }
        }
      }

      main { Outlet::<Routes> {} }

      div { id: "footer",
        button { onclick: fetch_new_quote, "Nouvelle citation" }
        p { "{quote}" }
      }
    }
}
