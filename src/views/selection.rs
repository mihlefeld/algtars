use std::{
    collections::{BTreeMap, HashMap},
    str::FromStr,
};

use dioxus::{html::option::selected, logger::tracing, prelude::*};
use wasm_bindgen::JsValue;
use web_sys::{js_sys::Array, Blob, BlobPropertyBag, Url};

// grid-cols-1
// grid-cols-2
// grid-cols-3
// grid-cols-4
// grid-cols-5
// grid-cols-6
// col-span-1
// col-span-2
// col-span-3
// col-span-4
// col-span-5
// col-span-6

pub fn get_bg_hover(x: bool) -> (String, String) {
    let bg = if x { "bg-blue-300" } else { "bg-red-300" };
    let hover = if x {
        "hover:bg-blue-200"
    } else {
        "hover:bg-red-200"
    };
    (bg.to_string(), hover.to_string())
}

#[component]
pub fn Case(selections: Signal<HashMap<String, bool>>, url: String) -> Element {
    let (bg, hover) = get_bg_hover(*selections().get(&url).unwrap());
    rsx! {
        div {
            class: "w-40 {bg} rounded-lg {hover}",
            onclick: {
                let url_cloned = url.clone();
                move |_| {
                    selections.write().entry(url_cloned.clone()).and_modify(|value| *value = !*value).or_insert(false);
                }
            },
            img {
                src: url.clone().as_str(),
                key: k
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct GroupProps {
    name: String,
    urls: Vec<String>,
}

#[component]
pub fn Group(props: GroupProps) -> Element {
    let name = props.name;
    let urls = props.urls;
    let mut selections = use_signal(|| {
        urls.iter()
            .map(|url| (url.clone(), false))
            .collect::<HashMap<String, bool>>()
    });
    let group_selected = use_memo(move || selections().iter().map(|(_, v)| v).all(|v| *v));
    let (bg, hover) = get_bg_hover(group_selected());
    let cols = format!("grid-cols-{}", if urls.len() <= 6 { urls.len() } else { 6 });
    let col_span = format!("col-span-{}", if urls.len() <= 6 { urls.len() } else { 6 });
    rsx! {
        div {
            class: "grid {cols} gap-1 w-fit",
            div {
                class: "{col_span}",
                onclick: {
                    move |_| {
                        for (_, val) in selections.write().iter_mut() {
                            *val = !group_selected();
                        }
                    }
                },
                class: "{bg} {hover} rounded-lg p-2",
                h2 {
                    class: "font-extrabold",
                    "{name}"
                },
            }
            for url in urls {
                Case {
                    selections: selections,
                    url:  url
                }
            }
        }
    }
}

// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Selection() -> Element {
    let (groups, blob_urls) = use_hook(|| {
        let blob_urls: HashMap<String, String> = serde_json::from_str::<BTreeMap<String, String>>(
            include_str!("../../assets/combined.json"),
        )
        .unwrap_or_else(|_| {
            tracing::debug!("Failed to load hashmap");
            BTreeMap::new()
        })
        .iter()
        .map(|(k, v)| {
            let options = BlobPropertyBag::new();
            options.set_type("image/svg+xml");
            let js_blob_array = Array::new();
            js_blob_array.push(&JsValue::from_str(v));
            let blob = Blob::new_with_str_sequence_and_options(&js_blob_array, &options).unwrap();
            let url = Url::create_object_url_with_blob(&blob).unwrap();
            tracing::debug!("created url for blob {} {}", k, url);
            (k.clone(), url)
        })
        .collect();

        let groups = serde_json::from_str(include_str!("../../assets/groups_info.json"))
            .unwrap_or_else(|_| {
                tracing::debug!("Failed to load groups");
                BTreeMap::<String, Vec<i32>>::new()
            })
            .iter()
            .map(|(name, group_ids)| {
                let urls = group_ids
                    .iter()
                    .map(|id| match blob_urls.get(&id.to_string()) {
                        Some(x) => x.clone(),
                        None => {
                            tracing::debug!("blob not found {}", id);
                            "".to_string()
                        }
                    })
                    .collect::<Vec<String>>();
                GroupProps {
                    name: name.to_string(),
                    urls,
                }
            })
            .collect::<Vec<GroupProps>>();
        (groups, blob_urls)
    });
    use_drop(move || {
        for (_, v) in blob_urls.iter() {
            tracing::debug!("dropped url {}", v);
            Url::revoke_object_url(v).ok();
        }
    });

    rsx! {
        div {
            class: "h-grow justify-center flex",
            div {
                class: "flex flex-col w-fit gap-1",
                for group in groups {
                    Group {
                        name: group.name,
                        urls: group.urls
                    }
                }
            }
        }
    }
}
