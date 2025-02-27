use dioxus::prelude::*;

use crate::layouts::IndexLayout;
use crate::pages::{Index, Rules};

#[derive(Routable, Clone, PartialEq)]
pub enum Routes {
    #[layout(IndexLayout)]
    #[route("/")]
    Index,
    #[route("/rules")]
    Rules,
}
