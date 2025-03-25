use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct DogAppProps {
    breed: String,
    age: u8,
}

// On every render, Dioxus makes a `.clone()` of the component's props.
#[component]
pub fn DogApp(props: DogAppProps) -> Element {
    rsx! {"Hello, {props.breed}!"}
}

pub fn HelloDiv() -> Element {
    let users = vec![
        User::new(1, "Alice".to_string()),
        User::new(2, "Bob".to_string()),
    ];
    rsx! {
        div { "Hello, 酷狗!" }
        for user in users {
            div {
                key: "{user.id}",
                "{user.name}"
            }
        }
    }
}

pub struct User {
    id: usize,
    name: String,
}

impl User {
    pub fn new(id: usize, name: String) -> Self {
        Self { id, name }
    }
}

#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            id: "title",
            h1 { "HotDog!" }
        }
        div {
            id: "dogview",
            img { src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg" }
        }
        div {
            id: "buttons",
            button { id: "skip", "skip"}
            button { id: "save", "save!"}
        }
    }
}
