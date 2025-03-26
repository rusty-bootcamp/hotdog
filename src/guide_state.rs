use dioxus::prelude::*;

use crate::{guide_assets::CSS, guide_fetching};

static SONG: GlobalSignal<String> = Signal::global(|| "Drift Away".to_string());

#[derive(Clone, Copy)]
struct TitleState {
    title: Signal<String>,
}

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
fn DogView() -> Element {
    let mut img_src = use_signal(|| "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg");
    let save = move |_| {
        consume_context::<TitleState>()
            .title
            .set("HotDog".to_string());
        img_src.set("https://images.dog.ceo/breeds/spaniel-blenheim/n02086646_3739.jpg");
    };
    let skip = move |_| {
        consume_context::<TitleState>()
            .title
            .set("Goose".to_string());
        img_src.set("https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg");
    };

    rsx! {
        div {
            id: "dogview",
            img { src: "{img_src}" }
        }
        div {
            id: "buttons",
            button {onclick: skip, id: "skip", "Skip"}
            button {onclick: save, id: "save", "Save"}
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
