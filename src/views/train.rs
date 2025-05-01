use crate::components::{Timer, Theme, DisplayTime};
use dioxus::prelude::*;


/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Train() -> Element {
    let theme = use_context::<Theme>();
    let final_time = use_signal(|| DisplayTime::new());
    let mut history = use_signal(|| Vec::<DisplayTime>::new());
    use_memo(move || {
        history.push(final_time())
    });

    rsx! {
        div {
            class: "{theme.accent_background} grid grid-cols-1 grid-rows-10 flex-grow p-2 row-span-8 gap-2",
            Timer { 
                class: "{theme.background} row-span-9 rounded-xl",
                final_time 
            }
            div {
                class: "{theme.background} flex row-span-1 gap-2 place-content-center rounded-xl",
                for t in history().iter() {
                    span {
                        "{t.display()}"
                    }
                }
            }
        }
    }
}
