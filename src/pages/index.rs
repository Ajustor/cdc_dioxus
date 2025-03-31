use dioxus::{html::ul, prelude::*};

struct Player {
    name: String,
    score: u8,
}

struct Game {
    name: String,
    players: Vec<Player>,
}

#[component]
pub fn Index() -> Element {
    let saved_games: Vec<Game> = Vec::new();

    rsx! {
      h1 { "Cul de chouette" }
      p { "Un calculateur de score pour le cul de chouette !" }

      div {
        ul {
                }
      }
    }
}
