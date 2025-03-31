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
    let mut quote = use_signal(|| "Réplique de Perceval".to_string());

    let fetch_new_quote = move |_| async move {
        quote.set("...".into());
        let response = reqwest::Client::new()
            .get("https://kaamelott.chaudie.re/api/random/personnage/Perceval")
            .header("Access-Control-Allow-Origin", "*")
            .header("Content-Type", "application/json")
            .send()
            .await;
        if response.is_ok() {
            let citation_body = response.unwrap().json::<KaamelotApi>().await.unwrap();
            quote.set(citation_body.citation.citation);
        } else {
            quote.set("Le pigeon s'est perdu en chemin".into());
        }
    };

    rsx! {
      // Global app resources
      document::Link { rel: "icon", href: FAVICON }
      document::Link { rel: "stylesheet", href: MAIN_CSS }

      nav { id: "header",
        if route == Routes::Rules {
          Link { to: Routes::Index, "<- Retour" }
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
