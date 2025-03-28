use dioxus::prelude::*;

#[component]
pub fn DogView() -> Element {
    let mut img_src = use_signal(|| "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg");
    let save = move |_| {
        img_src.set("https://images.dog.ceo/breeds/spaniel-blenheim/n02086646_3739.jpg");
    };
    let skip = move |_| {
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
pub fn PageNotFound(segments: Vec<String>) -> Element {
    rsx! {
        div {
            id: "notfound",
            h1 { "Page Not Found" }
        }
    }
}
