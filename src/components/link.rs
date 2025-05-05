use dioxus::prelude::*;
use crate::components::Theme;
use crate::Route;

#[component]
pub fn ALink(to: Route, name: &'static str, style: Option<String>) -> Element {
    let style = style.unwrap_or("w-50".to_string());
    let theme = use_context::<Theme>();
    rsx! {
        Link {
            class: "{style} font-bold {theme.button()}",
            to: to,
            "{name}"
        }
    }
}

#[component]
pub fn TextLink(to: Route, name: &'static str, style: Option<String>) -> Element {
    let style = style.unwrap_or("".to_string());
    let theme = use_context::<Theme>();
    rsx! {
        Link {
            class: "{style} {theme.text_link()}",
            to: to,
            "{name}"
        }
    }
}