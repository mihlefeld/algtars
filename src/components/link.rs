use dioxus::prelude::*;
use crate::components::Theme;
use crate::Route;

#[component]
pub fn ALink(to: Route, name: &'static str) -> Element {
    let theme = use_context::<Theme>();
    rsx! {
        Link {
            class: "{theme.secondary} {theme.secondary_hover} p-2 rounded-lg w-30 ",
            to: to,
            "{name}"
        }
    }
}