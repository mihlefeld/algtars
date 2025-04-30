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
    let mut visible = use_signal(|| false);
    let theme = use_context::<Theme>();
    let to_back_button = "font-black h-fit";
    // let classes = use_memo(move || "{theme.background} {theme.secondary} {theme.secondary_hover} p-1 rounded-lg");
    rsx! {
        div {
            class: "fixed top-0 min-h-screen flex",
            button {  
                class: "fixed top-3 left-3 {to_back_button} {theme.button()}",
                onclick: move |_| visible.set(true),
                ">"
            },
            if visible() {
                div {
                    class: "flex flex-row gap-3 min-h-screen justify-start sticky p-2 top-0 text-center shadow-lg w-fit {theme.background}",
                    id: "navbar",
                    div {
                        class: "flex flex-col gap-1",
                        ALink {
                            to: Route::SelectionRoute { trainer: "2x2-EG".to_string() },
                            name: "2x2-EG"
                        },
                        ALink {
                            to: Route::SelectionRoute { trainer: "2x2-FH".to_string() },
                            name: "2x2-FH"
                        },
                        ALink {
                            to: Route::SelectionRoute { trainer: "2x2-LS".to_string() },
                            name: "2x2-LS"
                        },
                        ALink {
                            to: Route::SelectionRoute { trainer: "Megaminx-PLL".to_string() },
                            name: "Megaminx-PLL"
                        },
                        ALink {
                            to: Route::SelectionRoute { trainer: "Megaminx-OLL".to_string() },
                            name: "Megaminx-OLL"
                        },
                        ALink {
                            to: Route::SelectionRoute { trainer: "3x3-ZBLL".to_string() },
                            name: "3x3-ZBLL",
                        },
                        ALink {
                            to: Route::Practice {},
                            name: "Practice"
                        },
                    }
                    button {  
                        class: "{to_back_button} {theme.button()}",
                        onclick: move |_| visible.set(false),
                        "<"
                    },
                },
                div {
                    onclick: move |_| visible.set(false),
                    class: "bg-black/75 w-screen h-screen cursor-pointer"
                }
            }
        }
        // The `Outlet` component is used to render the next component inside the layout. In this case, it will render either
        // the [`Home`] or [`Blog`] component depending on the current route.
        div {
            class: "w-screen flex justify-center min-h-screen",
            Outlet::<Route> {}
        }
        
    }
}
