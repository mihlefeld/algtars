use crate::views::TRAINERS;
use crate::{
    components::{ALink, Theme},
    Route,
};
use dioxus::prelude::*;

#[component]
fn SelectionLink(trainer: &'static str) -> Element {
    rsx! {
            ALink {
                to: Route::SelectionRoute { trainer: trainer.to_string() },
                name: trainer
            }

    }
}
#[component]
pub fn LinkTree(onclick: Option<EventHandler<MouseEvent>>) -> Element {
    let _theme = use_context::<Theme>();
    rsx! {
        div {
            class: "flex flex-col gap-1",
            for trainer in TRAINERS {
                div { 
                    class: "flex ",
                    onclick: move |m| {match onclick {
                        Some(f) => f(m),
                        None => ()
                    }},
                    SelectionLink { trainer, key: "{trainer}" }
                }
            }
        }
    }
}
