use crate::views::TRAINERS;
use crate::{
    components::{ALink, Theme},
    Route,
};
use dioxus::prelude::*;

#[component]
fn SelectionLink(trainer: &'static str, style: Option<String>) -> Element {
    rsx! {
            ALink {
                to: Route::SelectionRoute { trainer: trainer.to_string()},
                name: trainer,
                style: style,
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
                    class: "flex w-fit",
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

#[component]
pub fn LandingPage() -> Element {
    let _theme = use_context::<Theme>();
    rsx! {
        div {
            class: "flex flex-row gap-1 place-items-center pt-2 justify-center flex-wrap",
            for trainer in TRAINERS {
                div { 
                    class: "flex w-fit",
                    SelectionLink { trainer, key: "{trainer}", style: "" }
                }
            }
        }
    }
}