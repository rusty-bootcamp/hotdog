use dioxus::prelude::*;

use crate::{components::TitleState, guide_assets::CSS, guide_fetching};

static SONG: GlobalSignal<String> = Signal::global(|| "Drift Away".to_string());

#[component]
pub fn App() -> Element {
    // Provide TitleState type as a Context
    let title = use_signal(|| "HotDog".to_string());
    use_context_provider(|| TitleState { title });
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
    let state = use_context::<TitleState>();
    rsx! {
        div {
            id: "title",
            h1 { "{state.title}" }
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
