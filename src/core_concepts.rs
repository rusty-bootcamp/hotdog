use dioxus::prelude::*;
use rand::Rng;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

struct User {
    name: String,
    age: u32,
}

#[component]
fn App() -> Element {
    let mut user = use_signal(|| User {
        name: "John Doe".to_string(),
        age: 27,
    });

    let number_type = use_signal(|| true);

    rsx! {
        "Hello {user.read().name}"
        ListItem {}
        UseHook {}
        h1 {}
        Rerenders {}
        Effect {}
        TogglesChild {}
        div {
            // display sets the layout of the element
            display: "flex",
            // justify-content centers the element horizontally
            justify_content: "center",
            input {
                value: if number_type() {
                    "{user.read().age}"
                } else {
                    "{user.read().name}"
                },
                type: if number_type() { "number" } else { "text" },
                oninput: move |evt| user.set(User {
                    name: evt.value(),
                    age: 30,
                })
            }
        }
    }
}

#[component]
fn ListItem() -> Element {
    let items = use_signal(|| vec!["Tauri", "Dioxus"]);

    rsx! {
        ul {
            for item in items.iter() {
                li { "{item}" }
            }
        }
    }
}

#[component]
fn UseHook() -> Element {
    // The clousre that is passed to use_hook will be called
    // once the first time the component is rendered
    let random_number = use_hook(|| {
        let mut rng = rand::thread_rng();
        let new_random_number = rng.gen_range(0..100);
        log::info!("{new_random_number}");
        new_random_number
    });

    rsx! {
        div { "Random {random_number}" }
    }
}

#[component]
fn Rerenders() -> Element {
    let mut count = use_signal(|| 0);
    log::info!("Rerendering parent component with {}", *count.peek());

    rsx! {
        button {
            onclick: move |_| count += 1, "Increment"
        }
        // Since we read count here, the component will rerender when count changes
        Count { current_count: count() }
        button {
            onclick: move |_| count -= 1, "Decrement"
        }
    }
}

#[component]
fn Count(current_count: i32) -> Element {
    log::info!("Rerendering child component with {current_count}");

    rsx! {
        div { "The count is {current_count}" }
    }
}

#[component]
fn Effect() -> Element {
    // Effects run after the component is rendered
    // You can use them to read or modify the rendered component
    use_effect(|| {
        log::info!("Effect ran!");
        document::eval(&format!(
            "document.getElementById('effect-output').innerText = 'Rakudo Star'"
        ));
    });

    rsx! {
        div {id: "effect-output", "This will be updated by the effect"}
    }
}

#[component]
fn TogglesChild() -> Element {
    let mut toggled = use_signal(|| true);

    rsx! {
        button {
            onclick: move |_| toggled.toggle(),
            "Toggle"
        }
        if toggled() {
            Child {}
        }
    }
}

#[component]
fn Child() -> Element {
    // You can use the use_drop hook to clean up any resources
    use_drop(|| {
        log::info!("Child dropped!");
    });
    rsx! {
        div { "Child Component" }
    }
}
