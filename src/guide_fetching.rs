use dioxus::prelude::*;

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[component]
pub fn DogView() -> Element {
    let mut img_src = use_signal(|| "".to_string());

    let fetch_dog = move |_| async move {
        let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap();

        img_src.set(response.message);
    };

    rsx! {
        div {
            id: "dogview",
            img { src: "{img_src}" }
        }
        div {
            id: "buttons",
            button {onclick: fetch_dog, id: "save", "save!"}
        }
    }
}
