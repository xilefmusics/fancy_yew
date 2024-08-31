use super::{NumberInput, StringInput};
use std::collections::HashMap;
use stylist::Style;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub bind_handle: UseStateHandle<HashMap<String, f64>>,
    #[prop_or_default]
    pub min: Option<f64>,
    #[prop_or_default]
    pub max: Option<f64>,
    #[prop_or_default]
    pub options: Vec<String>,
    #[prop_or_default]
    pub strict: bool,
}

#[function_component(StringNumberMap)]
pub fn string_number_map(props: &Props) -> Html {
    let next_key = use_state(|| String::new());
    let next_value = use_state(|| 0.);
    let next_min = props.min;
    let next_max: Option<f64> = props
        .max
        .map(|max| max - (*props.bind_handle).values().sum::<f64>());
    let bind_handle = props.bind_handle.clone();
    let list = (*bind_handle)
        .iter()
        .map(|(key, value)| (key.to_string(), *value))
        .collect::<Vec<(String, f64)>>()
        .into_iter()
        .map(|(key, value)| {
            let ondelete = {
                let map_handle = bind_handle.clone();
                let key = key.clone();
                move |_: MouseEvent| {
                    let mut map = (*map_handle).clone();
                    map.remove(&key);
                    map_handle.set(map);
                }
            };
            html! {
                <tr>
                    <td><span class="list-item">{key.clone()}</span></td>
                    <td><span class="list-item">{value}</span></td>
                    <td><button onclick={ondelete}><span class="material-symbols-outlined icon">{"delete"}</span></button></td>
                </tr>
            }
        })
        .collect::<Vec<Html>>();
    let max = props.max;

    let onadd = {
        let map_handle = props.bind_handle.clone();
        let next_key = next_key.clone();
        let next_value = next_value.clone();
        move |_| {
            if (*next_key).len() == 0 {
                return;
            }
            let mut map = (*map_handle).clone();
            if let Some(max) = max {
                if map.values().sum::<f64>() + *next_value > max {
                    return;
                }
            }
            map.insert((*next_key).clone(), *next_value);
            map_handle.set(map);
            next_key.set(String::new());
            next_value.set(0.);
        }
    };
    html! {
        <table class={Style::new(include_str!("string_number_map.css")).expect("Unwrapping CSS should work!")}>
            {list}
            <tr>
                <td>
                    <StringInput
                        bind_handle={next_key}
                        options={props.options.clone()}
                        strict={props.strict}
                    />
                </td>
                <td>
                    <NumberInput
                        bind_handle={next_value}
                        min={next_min}
                        max={next_max}
                    />
                </td>
                <td><button onclick={onadd}><span class="material-symbols-outlined icon">{"add"}</span></button></td>
            </tr>
        </table>
    }
}
