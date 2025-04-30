use dioxus::prelude::*;
use crate::components::Theme;
use crate::Route;

#[component]
pub fn ALink(to: Route, name: &'static str) -> Element {
    let theme = use_context::<Theme>();
    rsx! {
        Link {
            class: "w-50 font-bold {theme.button()}",
            to: to,
            "{name}"
        }
    }
}