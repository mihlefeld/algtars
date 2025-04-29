use std::time::Duration;

use dioxus::{logger::tracing, prelude::*};
use dioxus_sdk::utils::timing::use_interval;
use super::Theme;

#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Hash, Default, Debug)]
pub struct DisplayTime {
    pub millis: i64,
}

impl DisplayTime {
    pub fn new() -> Self {
        Self { millis: 0 }
    }
    pub fn display(&self) -> String {
        let millis_in_hour = 1000 * 60 * 60;
        let millis_in_minute = millis_in_hour / 60;
        let millis_in_second = millis_in_minute / 60;
        let mut millis = self.millis;
        let hours = millis / millis_in_hour;
        millis -= hours * millis_in_hour;
        let minutes = millis / millis_in_minute;
        millis -= minutes * millis_in_minute;
        let seconds = millis / millis_in_second;
        millis -= seconds * millis_in_second;
        return match (hours, minutes, seconds) {
            (0, 0, 0) => format!("0.{:03}", millis),
            (0, 0, s) => format!("{}.{:03}", s, millis),
            (0, m, s) => format!("{}:{:02}.{:03}", m, s, millis),
            (h, m, s) => format!("{}:{:02}:{:02}.{:03}", h, m, s, millis),
        };
    }
}


#[component]
pub fn Timer(mut final_time: Signal<DisplayTime>, class: ReadOnlySignal<String>) -> Element {
    let mut timer_time = use_signal(|| DisplayTime::new());
    let mut start_time = use_signal(|| 0i64);
    let mut running = use_signal(|| false);
    use_interval(Duration::from_millis(10), {
        move || match running() {
            true => timer_time.set(DisplayTime {
                millis: chrono::Utc::now().timestamp_millis() - start_time(),
            }),
            false => (),
        }
    });
    
    let theme = use_context::<Theme>();

    let toggle = {
        move |event: Event<MouseData>| {
            event.prevent_default();
            tracing::debug!("called toggle");
            match running() {
                true => {
                    let done_time = DisplayTime {
                        millis: chrono::Utc::now().timestamp_millis() - start_time(),
                    };
                    timer_time.set(done_time);
                    final_time.set(done_time);
                    running.set(false);
                }
                false => {
                    start_time.set(chrono::Utc::now().timestamp_millis());
                    timer_time.set(DisplayTime::new());
                    running.set(true);
                }
            }
        }
    };

    rsx! {
        div {
            class: "cursor-pointer place-content-center place-items-center flex select-none {class}",
            onclick: toggle,
            p {
                class: "text-4xl",
                "{timer_time().display()}"
            }
        }
    }
}