use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub bind_handle: UseStateHandle<f64>,
    #[prop_or_default]
    pub min: Option<f64>,
    #[prop_or_default]
    pub max: Option<f64>,
}

#[function_component(NumberInput)]
pub fn number_input(props: &Props) -> Html {
    let oninput = {
        let bind_handle = props.bind_handle.clone();
        move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            bind_handle.set(input.value_as_number())
        }
    };

    if let Some(min) = props.min {
        if let Some(max) = props.max {
            html! {
                <input
                    min={min.to_string()}
                    max={max.to_string()}
                    value={props.bind_handle.to_string()}
                    oninput={oninput}
                    type="number"
                />
            }
        } else {
            html! {
                <input
                    min={min.to_string()}
                    value={props.bind_handle.to_string()}
                    oninput={oninput}
                    type="number"
                />
            }
        }
    } else {
        if let Some(max) = props.max {
            html! {
                <input
                    max={max.to_string()}
                    value={props.bind_handle.to_string()}
                    oninput={oninput}
                    type="number"
                />
            }
        } else {
            html! {
                <input
                    value={props.bind_handle.to_string()}
                    oninput={oninput}
                    type="number"
                />
            }
        }
    }
}
