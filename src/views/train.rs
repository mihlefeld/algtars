use crate::components::{Timer, Theme, DisplayTime};
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
            class: "w-screen {theme.accent_background} grid grid-cols-4 grid-rows-11 h-screen gap-2 flex flex-col p-2",
            div {
                class: "pl-10 col-span-4 row-span-1 {container_style}",
                "{scramble}"
            },
            Timer { 
                class: "{container_style} row-span-9 col-span-3",
                final_time 
            },
            div {
                class: "flex flex-col row-span-1 gap-2 place-content-center row-span-9 overflow-auto {container_style}",
                for t in history.iter() {
                    span {
                        "{t.display().unwrap()}"
                    }
                }
            },
            div {
                class: "col-span-4 row-span-1 {container_style}",
                "Test"
            },
        }
    }
}
