use dioxus::prelude::*;
use hot_dog::{
    guide_component::{DogApp, Header, HelloDiv},
    guide_state::App,
};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}
