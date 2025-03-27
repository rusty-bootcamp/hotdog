use dioxus::prelude::*;

use crate::guide_router::Route;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div {
            id: "title",
            Link {
                to: Route::DogView,
                h1 {"HotDog!"}
            }
            Link {
                to: Route::Favorites,
                id: "heart",
                "♥️"
            }
        }
        Outlet::<Route> {}
    }
}
