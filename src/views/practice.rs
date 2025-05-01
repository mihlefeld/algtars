use crate::components::{self, ALink, TextLink};
use crate::views::{get_selection_data, SelectionDataJson};
use crate::Route;
use crate::views::Train;
use dioxus::{CapturedError, Ok};
use dioxus::{logger::tracing, prelude::*};
use dioxus_radio::prelude::*;
use dioxus_sdk::storage::{get_from_storage, new_synced_storage, LocalStorage};
use core::fmt;
use std::collections::BTreeMap;
use wasm_bindgen::JsValue;
use web_sys::{js_sys::Array, Blob, BlobPropertyBag, Url};

#[derive(Default)]
struct SelectionData {
    pub selections: BTreeMap<i32, bool>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum SelectionDataChannel {
    All,
    InitSelection,
    SelectionChanged(i32),
    GroupSelectionChanged(String),
}

impl RadioChannel<SelectionData> for SelectionDataChannel {}

#[component]
pub fn Case(group: String, url: String, case_id: i32) -> Element {
    let mut persistent = use_context::<Signal<BTreeMap<i32, bool>>>();
    let mut radio = use_radio::<SelectionData, SelectionDataChannel>(
        SelectionDataChannel::SelectionChanged(case_id),
    );
    let theme = use_context::<components::Theme>();
    let bg_hover =
        use_memo(move || theme.selected_style(*radio.read().selections.get(&case_id).unwrap()));
    rsx! {
        div {
            class: "w-22 min-[33rem]:w-28 rounded-lg cursor-pointer p-1 {bg_hover}",
            onclick: {
                move |_| {
                    radio.write().selections.entry(case_id).and_modify(|value| *value = !*value).or_insert(false);
                    persistent.write().entry(case_id).and_modify(|value| *value = !*value).or_insert(false);
                    radio.write_channel(SelectionDataChannel::GroupSelectionChanged(group.clone()));
                    radio.write_channel(SelectionDataChannel::All);
                }
            },
            img {
                src: url.clone().as_str(),
                draggable: false,
                key: k
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct GroupProps {
    name: ReadOnlySignal<String>,
    urls: ReadOnlySignal<Vec<String>>,
    case_ids: ReadOnlySignal<Vec<i32>>,
}

#[component]
pub fn Group(props: GroupProps) -> Element {
    let mut persistent = use_context::<Signal<BTreeMap<i32, bool>>>();
    let small_bp = 4usize;
    let medium_bp = 6usize;
    let large_bp = 8usize;
    let name = props.name;
    let urls = props.urls;
    let case_ids = props.case_ids;

    let mut radio = use_radio(SelectionDataChannel::GroupSelectionChanged(name()));
    let group_selected = use_memo(move || {
        case_ids
            .read()
            .iter()
            .map(|case_id| radio.read().selections.get(case_id).cloned())
            .all(|x| x.unwrap_or(false))
    });
    let theme = use_context::<components::Theme>();
    let bg_hover = use_memo(move || theme.selected_style(group_selected()));

    let col_format = |prefix: &str, pb: usize| {
        urls.with(|urls| {
            format!(
                "{}-{}",
                prefix,
                if urls.len() <= pb { urls.len() } else { pb }
            )
        })
    };
    let grid_cols = format!(
        "{} {} {}",
        col_format("grid-cols", small_bp),
        col_format("min-[47rem]:grid-cols", medium_bp),
        col_format("min-[60rem]:grid-cols", large_bp)
    );
    let col_span = format!(
        "{} {} {}",
        col_format("col-span", small_bp),
        col_format("min-[47rem]:col-span", medium_bp),
        col_format("min-[60rem]:col-span", large_bp),
    );
    rsx! {
        div {
            class: "grid {grid_cols} gap-1 w-fit select-none",
            div {
                onclick: {
                    move |_| {
                        for i in case_ids.read().iter() {
                            radio.write_channel(SelectionDataChannel::SelectionChanged(*i)).selections.entry(*i).and_modify(|s| *s = !group_selected()).or_insert(!group_selected());
                            persistent.write().entry(*i).and_modify(|s| *s = !group_selected()).or_insert(!group_selected());
                        }
                        radio.write();
                        radio.write_channel(SelectionDataChannel::All);
                    }
                },
                class: "rounded-lg p-2 cursor-pointer {bg_hover} {col_span}",
                h2 {
                    class: "font-extrabold text-2xl",
                    "{name}"
                },
            }
            for (i, url) in urls().iter().enumerate() {
                Case {
                    group: name,
                    url:  url,
                    case_id: case_ids.read()[i],
                }
            }
        }
    }
}

#[component]
pub fn Selection(groups: ReadOnlySignal<Vec<GroupProps>>) -> Element {
    rsx! {
        div {
            class: "flex flex-col w-fit gap-1 place-self-center pb-40",
            for group in groups.read().iter() {
                Group {
                    name: group.name,
                    urls: group.urls,
                    case_ids: group.case_ids,
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum PracticeMode {
    Recap,
    Train,
    Select,
}


impl std::str::FromStr for PracticeMode {
    type Err = CapturedError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "recap" => Ok(PracticeMode::Recap),
            "train" => Ok(PracticeMode::Train),
            "select" => Ok(PracticeMode::Select),
            _ => Err(CapturedError::from_str("No such mode").unwrap())
        }
    }
}

impl std::fmt::Display for PracticeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[component]
pub fn Practice(trainer: String, mode: ReadOnlySignal<PracticeMode>) -> Element {
    // use local storage to persist the case selections TODO: make Case Selections a struct
    let mut persistent_selection = use_context_provider(|| {
        let key = trainer.clone() + "-selection";
        new_synced_storage::<LocalStorage, BTreeMap<i32, bool>>(key.clone(), || {
            get_from_storage::<LocalStorage, BTreeMap<i32, bool>>(key, BTreeMap::new)
        })
    });

    // initialize a radio station for fin grained reactivity in global state 
    use_init_radio_station::<SelectionData, SelectionDataChannel>(SelectionData::default);
    let mut radio =
        use_radio::<SelectionData, SelectionDataChannel>(SelectionDataChannel::InitSelection);

    // initialize the groups and blob urls
    // blob urls are urls to the svg images used in <img> tag
    let (groups, blob_urls) = use_hook(|| {

        // first retrieve the json fiels for the current alg trainer
        let SelectionDataJson {
            _algs: _,
            _algsets: _,
            images,
            groups,
            _scrambles: _,
        } = get_selection_data(trainer.as_str()).unwrap();

        // create blob urls from the svg images in compined.json
        let blob_urls: BTreeMap<String, String> =
            serde_json::from_str::<BTreeMap<String, String>>(images.as_str())
                .unwrap_or_else(|_| {
                    tracing::debug!("Failed to load hashmap");
                    BTreeMap::new()
                })
                .iter()
                .map(|(k, v)| {
                    // create blob and associate object url with it
                    let options = BlobPropertyBag::new();
                    options.set_type("image/svg+xml");
                    let js_blob_array = Array::new();
                    js_blob_array.push(&JsValue::from_str(v));
                    let blob =
                        Blob::new_with_str_sequence_and_options(&js_blob_array, &options).unwrap();
                    let url = Url::create_object_url_with_blob(&blob).unwrap();
                    (k.clone(), url)
                })
                .collect();

        // initialize vector of group props from groups_info.json and persistent storage
        // insert case selections into radio
        let groups = serde_json::from_str(groups.as_str())
            .unwrap_or_else(|_| {
                tracing::debug!("Failed to load groups");
                BTreeMap::<String, Vec<i32>>::new()
            })
            .iter()
            .map(|(name, group_ids)| {
                // get urls of the group
                let urls = group_ids
                    .iter()
                    .map(|id| match blob_urls.get(&id.to_string()) {
                        // check that every case id also has a blob url, usually shouldn't be a problem
                        Some(x) => x.clone(),
                        None => {
                            tracing::debug!("blob not found {}", id);
                            "".to_string()
                        }
                    })
                    .collect::<Vec<String>>();
                
                // insert selections into persistent state and radio
                for case_id in group_ids.iter() {
                    let value = persistent_selection
                        .write()
                        .entry(*case_id)
                        .or_insert(false).clone();
                    radio
                        .write_channel(SelectionDataChannel::SelectionChanged(*case_id))
                        .selections
                        .insert(*case_id, value);
                }

                // return group props 
                GroupProps {
                    name: ReadOnlySignal::new(Signal::new(name.to_string())),
                    urls: ReadOnlySignal::new(Signal::new(urls)),
                    case_ids: ReadOnlySignal::new(Signal::new(group_ids.clone())),
                }
            })
            .collect::<Vec<GroupProps>>();
        (groups, blob_urls)
    });

    // clean up the blob urls if a different alg trainer is chosen
    use_drop(move || {
        for (_, v) in blob_urls.iter() {
            Url::revoke_object_url(v).ok();
        }
    });

    rsx! {
        div {
            class: "flex flex-col w-screen gap-1 place-items-center",
            if mode == PracticeMode::Select {
                ALink { 
                    to: Route::SelectionRouteWithMode { trainer: trainer.clone(), mode: PracticeMode::Train },
                    name: ">",
                    style: "w-10 fixed right-2 bottom-2"
                },
                div { 
                    class: "black text-2xl",
                    h1 {
                        "{trainer}"
                    },
                    TextLink { to: Route::LandingPage {  }, name: "Back" }
                 },
                Selection { groups }
            } else {
                ALink { 
                    to: Route::SelectionRouteWithMode { trainer: trainer.clone(), mode: PracticeMode::Select },
                    name: "<",
                    style: "w-10 fixed right-2 bottom-2"
                },
                Train {}
            }
        }
    }
}

#[component]
pub fn SelectionRoute(trainer: ReadOnlySignal<String>) -> Element {
    rsx! {
        for i in std::iter::once(trainer) {
            Practice { trainer: trainer, key: "{i}", mode: ReadOnlySignal::new(Signal::new(PracticeMode::Select)) }
        }
    }
}
#[component]
pub fn SelectionRouteWithMode(trainer: ReadOnlySignal<String>, mode: ReadOnlySignal<PracticeMode>) -> Element {
    rsx! {
        for i in std::iter::once(trainer) {
            Practice { trainer: trainer, key: "{i}", mode }
        }
    }
}
