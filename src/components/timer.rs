use dioxus::prelude::*;
// use dioxus_time::

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
pub fn Timer() -> Element {
    rsx! {

    }
}