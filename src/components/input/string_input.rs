use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub bind_handle: UseStateHandle<String>,
    #[prop_or_default]
    pub options: Vec<String>,
    #[prop_or_default]
    pub strict: bool,
    #[prop_or_default]
    pub callback: Option<Callback<String>>,
}

#[function_component]
pub fn StringInput(props: &Props) -> Html {
    let oninput = {
        let bind_handle = props.bind_handle.clone();
        let callback = props.callback.clone();
        move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            bind_handle.set(input.value());
            if let Some(callback) = callback.clone() {
                callback.emit(input.value());
            }
        }
    };
    let onchange = {
        let bind_handle = props.bind_handle.clone();
        let callback = props.callback.clone();
        move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            bind_handle.set(input.value());
            if let Some(callback) = callback.clone() {
                callback.emit(input.value());
            }
        }
    };
    let value = (*props.bind_handle).clone();

    if props.options.len() > 0 {
        let mut options = props
            .options
            .iter()
            .cloned()
            .filter(|s| *s != value)
            .collect::<Vec<String>>();

        options.push(value.clone());
        let options = options
            .iter()
            .map(|option| html! {<option value={option.clone()}>{option.clone()}</option>})
            .collect::<Html>();
        if props.strict {
            html! {
            <select onchange={onchange}>
                {options}
            </select>
                }
        } else {
            let list_id = format!("list-{}", crate::generate_random_string(10));
            html! {
                <>
                    <input
                        list={list_id.clone()}
                        value={value}
                        oninput={oninput}
                        type="string"
                    />
                    <datalist id={list_id}>
                        {options}
                    </datalist>
                </>
            }
        }
    } else {
        html! {
            <input
                value={value}
                oninput={oninput}
                type="string"
            />
        }
    }
}
