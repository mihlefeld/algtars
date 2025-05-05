use std::time::Duration;

use dioxus::{logger::tracing, prelude::*};
use dioxus_radio::prelude::*;
use dioxus_sdk::utils::timing::use_interval;
use super::Theme;


#[derive(PartialEq, Eq, Clone, Debug)]
pub enum TimerChannel {
    Stopped,
}

impl RadioChannel<()> for TimerChannel {}


#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Hash, Default, Debug)]
pub struct DisplayTime {
    pub millis: Option<i64>,
}

impl DisplayTime {
    pub fn new() -> Self {
        Self { millis: None }
    }
    pub fn display(&self) -> Option<String> {
        let mut millis = self.millis?;
        let millis_in_hour: i64 = 1000 * 60 * 60;
        let millis_in_minute = millis_in_hour / 60;
        let millis_in_second = millis_in_minute / 60;
        let hours = millis / millis_in_hour;
        millis -= hours * millis_in_hour;
        let minutes = millis / millis_in_minute;
        millis -= minutes * millis_in_minute;
        let seconds = millis / millis_in_second;
        millis -= seconds * millis_in_second;
        Some(match (hours, minutes, seconds) {
            (0, 0, 0) => format!("0.{:03}", millis),
            (0, 0, s) => format!("{}.{:03}", s, millis),
            (0, m, s) => format!("{}:{:02}.{:03}", m, s, millis),
            (h, m, s) => format!("{}:{:02}:{:02}.{:03}", h, m, s, millis),
        })
    }

    pub fn valid(&self) -> bool {
        return self.millis.is_some();
    }
}


#[component]
pub fn Timer(mut final_time: Signal<DisplayTime>, class: ReadOnlySignal<String>) -> Element {
    let mut timer_time = use_signal(|| DisplayTime::new());
    let mut start_time = use_signal(|| 0i64);
    let mut running = use_signal(|| false);
    let mut radio = use_radio(TimerChannel::Stopped);
    use_interval(Duration::from_millis(10), {
        move || match running() {
            true => timer_time.set(DisplayTime {
                millis: Some(chrono::Utc::now().timestamp_millis() - start_time()),
            }),
            false => (),
        }
    });
    
    let _theme = use_context::<Theme>();

    let toggle = {
        move || {
            tracing::debug!("called toggle");
            match running() {
                true => {
                    let done_time = DisplayTime {
                        millis: Some(chrono::Utc::now().timestamp_millis() - start_time()),
                    };
                    timer_time.set(done_time);
                    final_time.set(done_time);
                    running.set(false);
                    radio.write();
                }
                false => {
                    start_time.set(chrono::Utc::now().timestamp_millis());
                    timer_time.set(DisplayTime::new());
                    running.set(true);
                }
            }
        }
    };

    let timer_string = use_memo(move || timer_time.read().display().unwrap_or("0.00".to_string()));
    
    rsx! {
        div {
            tabindex: 0,
            class: "cursor-pointer place-content-center place-items-center flex select-none {class}",
            onclick: move |m| {m.prevent_default(); toggle.clone()();},
            onkeypress: move |k| match k.code() {
                Code::Space => {
                    k.prevent_default();
                    toggle.clone()()
                },
                _ => ()
            },
            p {
                class: "text-8xl font-black",
                "{timer_string}"
            }
        }
    }
}