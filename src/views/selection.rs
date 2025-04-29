use std::collections::{BTreeMap, HashMap};

use dioxus::{logger::tracing, prelude::*};
use wasm_bindgen::JsValue;
use web_sys::{js_sys::Array, Blob, BlobPropertyBag, Url};

use crate::components;


pub struct SelectionDataJson {
    _algs: String,
    _algsets: String,
    images: String,
    groups: String,
    _scrambles: String,
}

pub fn get_selection_data(trainer: &str) -> Option<SelectionDataJson> {
    match trainer {
        "Megaminx-PLL" => Some(SelectionDataJson { 
            _algs: include_str!("../../assets/Megaminx/PLL/algs_info.json").to_string(), 
            _algsets: include_str!("../../assets/Megaminx/PLL/algsets_info.json").to_string(), 
            images: include_str!("../../assets/Megaminx/PLL/combined.json").to_string(), 
            groups: include_str!("../../assets/Megaminx/PLL/groups_info.json").to_string(), 
            _scrambles: include_str!("../../assets/Megaminx/PLL/selected_algsets.json").to_string() 
        }),
        "Megaminx-OLL" => Some(SelectionDataJson { 
            _algs: include_str!("../../assets/Megaminx/OLL/algs_info.json").to_string(), 
            _algsets: include_str!("../../assets/Megaminx/OLL/algsets_info.json").to_string(), 
            images: include_str!("../../assets/Megaminx/OLL/combined.json").to_string(), 
            groups: include_str!("../../assets/Megaminx/OLL/groups_info.json").to_string(), 
            _scrambles: include_str!("../../assets/Megaminx/OLL/selected_algsets.json").to_string() 
        }),
        "3x3-ZBLL" => Some(SelectionDataJson { 
            _algs: include_str!("../../assets/3x3/ZBLL/algs_info.json").to_string(), 
            _algsets: include_str!("../../assets/3x3/ZBLL/algsets_info.json").to_string(), 
            images: include_str!("../../assets/3x3/ZBLL/combined.json").to_string(), 
            groups: include_str!("../../assets/3x3/ZBLL/groups_info.json").to_string(), 
            _scrambles: include_str!("../../assets/3x3/ZBLL/selected_algsets.json").to_string() 
        }),
        _ => None
    }
}

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
// dark

#[component]
pub fn Case(selections: Signal<HashMap<String, bool>>, url: String) -> Element {
    let theme = use_context::<components::Theme>();
    let bg_hover = use_memo({ 
        let url = url.clone();
        move || theme.selected_style(*selections.read().get(&url.clone()).unwrap())
    });
    rsx! {
        div {
            class: "w-40 rounded-lg cursor-pointer {bg_hover}",
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
    name: ReadOnlySignal<String>,
    urls: ReadOnlySignal<Vec<String>>,
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
    let theme = use_context::<components::Theme>();
    let bg_hover = use_memo(move || {theme.selected_style(group_selected())});
    let cols = urls.with(|urls| format!("grid-cols-{}", if urls.len() <= 6 { urls.len() } else { 6 }));
    let col_span = urls.with(move |urls| format!("col-span-{}", if urls.len() <= 6 { urls.len() } else { 6 }));
    rsx! {
        div {
            class: "grid {cols} gap-1 w-fit pt-2",
            div {
                onclick: {
                    move |_| {
                        for (_, val) in selections.write().iter_mut() {
                            *val = !group_selected();
                        }
                    }
                },
                class: "rounded-lg p-2 cursor-pointer {bg_hover} {col_span}",
                h2 {
                    class: "font-extrabold",
                    "{name}"
                },
            }
            for url in urls() {
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
pub fn Selection(trainer: String) -> Element {
    let (groups, blob_urls) = use_hook(|| {
        let SelectionDataJson {_algs: _, _algsets: _, images, groups, _scrambles: _} = get_selection_data(trainer.as_str()).unwrap();
        let blob_urls: HashMap<String, String> = serde_json::from_str::<BTreeMap<String, String>>(
            images.as_str(),
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

        let groups = serde_json::from_str(groups.as_str())
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
                    name: ReadOnlySignal::new(Signal::new(name.to_string())),
                    urls: ReadOnlySignal::new(Signal::new(urls)),
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
            class: "flex flex-col w-fit gap-1 place-self-center",
            for group in groups {
                Group {
                    name: group.name,
                    urls: group.urls
                }
            }
        }
    
    }
}
