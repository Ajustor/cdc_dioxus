use dioxus::prelude::*;

#[component]
pub fn Rules() -> Element {
    rsx! {

      div {
        div {
          h2 { "== Origine du nom ==" }
          p {
            "Le score à atteindre (343 points) provient des initiales du nom du jeu: C-D-C, soit les 3e, 4e et 3e lettres de l'alphabet."
          }
        }
        div {
          h2 { "== Le jeu ==" }
          p {
            "Le Cul De Chouette se joue avec trois dés. Le but du jeu est d'atteindre le premier 343 points, en formant différentes combinaisons, chacun à son tour de jeu."
          }
        }
        div {
          h2 { "== Démarrage ==" }
          p {
            "Chaque joueur lance 1 dé, le joueur qui a fait le plus petit score commence. S'il y a plusieurs joueurs qui ont fait le plus petit score, ils recommencent pour se départager. Le tour de jeu s'effectue ensuite dans le sens inverse des aiguilles d'une montre."
          }
        }
        div {
          h2 { "== Déroulement du jeu ==" }
          p {
            "À son tour de jeu, le joueur lance d'abord 2 dés (les Chouettes), puis le 3e dé (le Cul). On applique alors la règle correspondant à la combinaison formée par les trois dés (cf. ci-dessous), sans tenir compte de l'ordre dans lequel les dés ont été tirés. On peut lancer les trois dés d'un coup pour aller plus vite et si les règles de l'Achat de Dé et de la Banque ne s'appliquent pas lors de la partie."
          }
        }
        div {
          h2 { "== Les combinaisons ==" }
          div {
            h3 { "La Chouette: 2 dés identiques" }
            p { "La Chouette a pour valeur le chiffre des deux dés identiques." }
            p {
              "Le joueur qui a lancé les dés gagne les points correspondant au carré de la Chouette:"
            }
            blockquote { class: "quote",
              "Chouette de 1 = 1 pt, de 2 = 4 pts, de 3 = 9 pts, de 4 = 16 pts, de 5 = 25 pts, de 6 = 36 pts"
            }
          }

          div {
            h3 { "La Velute: la somme de 2 dés = le 3e dé" }
            p { "La Velute a pour valeur le chiffre du 3e dé." }
            p {
              "Le joueur qui a lancé les dés gagne les points correspondant au double du carré de la Velute:"
            }
            blockquote { class: "quote",
              "Velute de 2 = 8 pts, de 3 = 18 pts, de 4 = 32 pts, de 5 = 50 pts, de 6 = 72 pts."
            }
          }

          div {
            h3 { "La Chouette Velute: une Chouette + une Velute" }
            p { "La Chouette Velute a la valeur de sa Velute." }
            p {
              "Le premier joueur qui frappe dans ses mains en criant \"Pas mou le caillou !\" gagne les points de laChouette Velute. . En cas d'égalité, les joueurs concernés (tous les joueurs qui ont fait l'enchaînement en même temps) perdent les points de la Chouette velute."
            }
          }

          div {
            h3 { "Le Cul de Chouette: 3 dés identique" }
            p {
              "Le Cul de chouette vaut une valeur fixe de 50 à 100 points en fonction du Cul de chouette:"
            }
            blockquote { class: "quote",
              "Cul de chouette de 1 = 50pts, de 2 = 60pts, de 3 = 70pts, de 4 = 80pts, de 5 = 90pts, de 6 = 100pts"
            }
          }

          div {
            h3 { "Suite: 3 dés se suivent" }
            p {
              "Dans ce cas, tous les joueurs doivent frapper du point sur la table et crier « Grelotte ça picote ». Le dernier à avoir fait cet enchaînement perd 10 points (en cas d'égalité, les joueurs sont départagés par un « Sans fin, est la moisissure des bières bretonnes »)."
            }
          }
        }

        div {
          h2 { "Régles additionnelles" }
          div {
            h3 { "Sirotage" }
            p {
              "Une fois les trois dés lancés, en cas de Chouette, le joueur a la possibilité de rejouer un dés pour obtenir un Cul de chouette correspondant à sa Chouette."
            }

            p {
              "Tous les joueurs parient alors sur le chiffre que va donner le dés. Si le joueur sirotant réussi son sirotage, il gagne son Cul de chouette, dans le cas contraire, la valeur de sa Chouette est soustraite de son score. Si un joueur tombe sur le bon chiffre, il gagne 25 points."
            }

            p { "Chaque nombre est annoncé par les joueurs de la façon suivante:" }

            ul {
              li { "Linotte: 1" }
              li { "Alouette: 2" }
              li { "Fauvette: 3" }
              li { "Mouette: 4" }
              li { "Bergeronnette: 5" }
              li { "Chouette: 6" }
            }

            blockquote { class: "quote",
              "Cul de chouette de 1 = 50pts, de 2 = 60pts, de 3 = 70pts, de 4 = 80pts, de 5 = 90pts, de 6 = 100pts"
            }
          }
        }
      }
    }
}
