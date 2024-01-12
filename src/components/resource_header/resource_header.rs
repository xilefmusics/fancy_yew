use crate::rest::Resource;
use serde::{de::DeserializeOwned, Serialize};
use stylist::Style;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props<T: PartialEq + DeserializeOwned + Serialize + Clone + Default> {
    pub handle: UseStateHandle<Resource<T>>,
}

#[function_component]
pub fn ResourceHeader<T: PartialEq + 'static + DeserializeOwned + Serialize + Clone + Default>(
    props: &Props<T>,
) -> Html {
    let handle = props.handle.clone();
    html! {
        <div class={Style::new(include_str!("resource_header.css")).expect("Unwrapping CSS should work!")}>
                <button
                    class="material-symbols-outlined icon"
                    onclick={Resource::add_default_closure::<MouseEvent>(handle.clone())}
                >{"add"}</button>
        </div>
    }
}
