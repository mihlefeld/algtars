use crate::components::{Timer, DisplayTime};
use dioxus::prelude::*;


/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let final_time = use_signal(|| DisplayTime::new());
    let mut history = use_signal(|| Vec::<DisplayTime>::new());
    use_memo(move || {
        history.push(final_time())
    });

    rsx! {
        div {
            class: "h-grow",
            Timer { final_time }
            div {
                class: "flex flex-row gap-2 place-content-center",
                for t in history().iter() {
                    span {
                        "{t.display()}"
                    }
                }
            }
        }
    }
}
