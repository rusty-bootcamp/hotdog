use dioxus::prelude::*;

use crate::{guide_assets::CSS, guide_fetching};

static SONG: GlobalSignal<String> = Signal::global(|| "Drift Away".to_string());

#[component]
pub fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Title {}
        // Player {}
        guide_fetching::DogView {}
    }
}

#[component]
fn Title() -> Element {
    // Consume TitleState as a Context
    rsx! {
        div {
            id: "title",
            h1 {  }
        }
    }
}

#[component]
fn Player() -> Element {
    rsx! {
        h3 { "Now playing {SONG}" }
        button {
            id: "player",
            onclick: move |_| *SONG.write() = "Vienna".to_string(),
            "Shuffle"
        }
    }
}
