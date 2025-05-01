use crate::Route;
use crate::components::{Theme, ALink};
use crate::views::LinkTree;
use dioxus::{logger::tracing,prelude::*};

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
                    class: "flex flex-row gap-3 h-screen overflow-auto justify-start sticky p-2 top-0 text-center shadow-lg w-fit {theme.background}",
                    key: "navbar",
                    LinkTree { onclick: move |_| {tracing::debug!("hide"); visible.set(false)} },
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
