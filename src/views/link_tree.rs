use dioxus::prelude::*;
use crate::{components::{ALink, Theme}, Route};

#[component]
pub fn LinkTree() -> Element {
    let _theme = use_context::<Theme>();
    rsx! {
        div {
            class: "flex flex-col gap-2 self-center",
            ALink { 
                to: Route::Selection { trainer: "Megaminx-PLL".to_string() },
                name: "Megaminx-PLL"
             },
            ALink { 
                to: Route::Selection { trainer: "Megaminx-OLL".to_string() },
                name: "Megaminx-OLL"
             },
            ALink { 
                to: Route::Selection { trainer: "3x3-ZBLL".to_string() },
                name: "3x3-ZBLL"
             },
        }
    }
}