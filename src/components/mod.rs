//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component  to be used in our app.

mod timer;
pub use timer::{DisplayTime, Timer, TimerChannel};

mod theme;
pub use theme::Theme;

mod link;
pub use link::{ALink, TextLink};