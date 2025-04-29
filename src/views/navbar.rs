use crate::Route;
use crate::components::{Theme, ALink};
use dioxus::prelude::*;
    
    /// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
    ///
    ///
    /// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar. The contents of the Home and Blog
    /// routes will be rendered under the outlet inside this component
#[component]
pub fn Navbar() -> Element {
    let theme = use_context::<Theme>();
    // let classes = use_memo(move || "{theme.background} {theme.secondary} {theme.secondary_hover} p-1 rounded-lg");
    rsx! {
        div {
            class: "w-full flex gap-3 justify-end sticky p-2 top-0 text-center shadow-lg {theme.background}",
            id: "navbar",
            ALink {
                to: Route::Selection { trainer: "Megaminx-PLL".to_string() },
                name: "Megaminx-PLL"
            },
            ALink {
                to: Route::Selection { trainer: "Megaminx-OLL".to_string() },
                name: "Megaminx-OLL"
            },
            ALink {
                to: Route::LinkTree {},
                name: "Links"
            },
            ALink {
                to: Route::Practice {},
                name: "Practice"
            },
        }

        // The `Outlet` component is used to render the next component inside the layout. In this case, it will render either
        // the [`Home`] or [`Blog`] component depending on the current route.
        Outlet::<Route> {}
    }
}
