use crate::components::input::FileInput;
use crate::rest::Resource;
use gloo::file::File;
use serde::{de::DeserializeOwned, Serialize};
use stylist::Style;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props<T: PartialEq + DeserializeOwned + Serialize + Clone + Default> {
    pub handle: UseStateHandle<Resource<T>>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn ResourceHeader<T: PartialEq + 'static + DeserializeOwned + Serialize + Clone + Default>(
    props: &Props<T>,
) -> Html {
    let handle = props.handle.clone();
    let files_handle = use_state(|| Vec::new());
    let files_handle_visible = use_state(|| false);

    let onimport_button = {
        let files_handle_visible = files_handle_visible.clone();
        move |_: MouseEvent| files_handle_visible.set(!*files_handle_visible)
    };

    let onimport = {
        let files_handle_visible = files_handle_visible.clone();
        let callback = Resource::import_callback(handle.clone());
        move |files: Vec<File>| {
            files_handle_visible.set(!*files_handle_visible);
            callback.emit(files);
        }
    };

    html! {
        <div class={Style::new(include_str!("resource_header.css")).expect("Unwrapping CSS should work!")}>
            {
                if *files_handle_visible {
                    html!{
                        <FileInput
                            bind_handle={files_handle}
                            callback={onimport}
                        />
                    }
                } else {
                    html!()
                }
            }
            {
                props.children.clone()
            }
            <button
                class="material-symbols-outlined icon"
                onclick={Resource::export_closure::<MouseEvent>(handle.clone())}
            >{"download"}</button>
            <button
                class="material-symbols-outlined icon"
                onclick={onimport_button}
            >{"upload"}</button>
            <button
                class="material-symbols-outlined icon"
                onclick={Resource::add_default_closure::<MouseEvent>(handle.clone())}
            >{"add"}</button>
        </div>
    }
}
