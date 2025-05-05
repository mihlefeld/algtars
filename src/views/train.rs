use crate::components::{DisplayTime, TextLink, Theme, Timer};
use crate::Route;
use dioxus::prelude::*;


/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Train(scramble: String, case: Signal<i32>) -> Element {
    let theme = use_context::<Theme>();
    let final_time = use_signal(|| DisplayTime::new());
    let mut history = use_signal(|| Vec::<DisplayTime>::new());
    use_memo(move || {
        if final_time.read().valid() {
            history.push(final_time());
        }
    });
    let container_style = format!("{} rounded-xl p-2", theme.background);
    rsx! {
        div {
            class: "w-screen {theme.accent_background} flex flex-col h-screen gap-2 flex flex-col p-2",
            p {
                class: "pl-10 text-4xl min-h-fit text-center font-black {container_style}",
                TextLink { to: Route::LandingPage {}, name: "Train", style: "font-black"},
                " 12 Cases"
            },
            div {
                class: "pl-10 text-4xl min-h-fit {container_style}",
                "{scramble}"
            },
            div {
                class: "flex gap-2 grow-1 overflow-hidden",
                Timer { 
                    class: "{container_style} text-7xl w-[70%]",
                    final_time 
                },
                div {
                    class: "flex flex-col gap-2 overflow-scroll w-[30%] {container_style}",
                    div {
                        class: "flex flex-col",
                        for t in history.iter() {
                            span {
                                "{t.display().unwrap()}"
                            }
                        }
                    }
                },
            }
            div {
                class: " {container_style}",
                "Test"
            },
        }
    }
}
