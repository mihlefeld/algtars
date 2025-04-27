use crate::Route;
use dioxus::prelude::*;


/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Navbar() -> Element {
    let classes = "bg-blue-300 hover:bg-blue-200 p-1 rounded-lg";
    rsx! {
        div {
            class: "w-full flex gap-3 justify-center sticky top-2",
            id: "navbar",
            Link {
                class: "{classes}",
                to: Route::Home {},
                "Home"
            },
            Link {
                class: "{classes}",
                to: Route::Selection {},
                "Selection"
            }
        }

        // The `Outlet` component is used to render the next component inside the layout. In this case, it will render either
        // the [`Home`] or [`Blog`] component depending on the current route.
        Outlet::<Route> {}
    }
}
