// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

use views::{Practice, Selection, Navbar, LinkTree};

/// Define a components module that contains all shared components for our app.
mod components;
/// Define a views module that contains the UI for all Layouts and Routes for our app.
mod views;

/// The Route enum is used to define the structure of internal routes in our app. All route enums need to derive
/// the [`Routable`] trait, which provides the necessary methods for the router to work.
/// 
/// Each variant represents a different URL pattern that can be matched by the router. If that pattern is matched,
/// the components for that route will be rendered.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        LinkTree {},
        #[route("/practice")]
        Practice {},
        #[route("/:trainer")]
        Selection { trainer: String },
}

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");
// The asset macro also minifies some assets like CSS and JS to make bundled smaller
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");



fn main() {
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    dioxus::launch(App);
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    use_context_provider(|| components::Theme{
        dark: true, 
        text: "text-[var(--text)]".to_string(),
        background: "bg-[var(--background)]".to_string(),
        accent_background: "bg-[var(--background-darker)]".to_string(),
        primary: "bg-[var(--primary)]".to_string(),
        primary_hover: "hover:bg-[var(--primary-hover)]".to_string(),
        secondary: "bg-[var(--secondary)]".to_string(),
        secondary_hover: "hover:bg-[var(--secondary-hover)]".to_string(),
        accent: "bg-[var(--accent)]".to_string(),
        accent_hover: "bg-[var(--accent-hover)]".to_string(),
    });
    let theme = use_context::<components::Theme>();
    let dark = if theme.dark {"dark".to_string()} else {"".to_string()};
    // The `rsx!` macro lets us define HTML inside of rust. It expands to an Element with all of our HTML inside.
    rsx! {
        // In addition to element and text (which we will see later), rsx can contain other components. In this case,
        // we are using the `document::Link` component to add a link to our favicon and main CSS file into the head of our app.
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        // The router component renders the route enum we defined above. It will handle synchronization of the URL and render
        // the layouts and components for the active route.
        div {
            class: "dark",
            div {
                class: "flex flex-col min-h-screen {dark} {theme.text} {theme.accent_background} gap-4",
                Router::<Route> {}
            }
        }
    }
}
