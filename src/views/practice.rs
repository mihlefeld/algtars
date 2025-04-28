use crate::components::{Timer, DisplayTime};
use dioxus::prelude::*;


/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Practice() -> Element {
    let final_time = use_signal(|| DisplayTime::new());
    let mut history = use_signal(|| Vec::<DisplayTime>::new());
    use_memo(move || {
        history.push(final_time())
    });

    rsx! {
        div {
            class: "grid grid-cols-1 grid-rows-10 flex-grow p-2 row-span-8 gap-2",
            Timer { 
                class: "row-span-9",
                final_time 
            }
            div {
                class: "flex row-span-1 gap-2 place-content-center",
                for t in history().iter() {
                    span {
                        "{t.display()}"
                    }
                }
            }
        }
    }
}
