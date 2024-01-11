use stylist::Style;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub bind_handle: UseStateHandle<bool>,
}

#[function_component]
pub fn BoolInput(props: &Props) -> Html {
    let oninput = {
        let bind_handle = props.bind_handle.clone();
        move |_: InputEvent| bind_handle.set(!*bind_handle)
    };

    html! {
        <label class={Style::new(include_str!("bool_input.css")).expect("Unwrapping CSS should work!")}>
            <input
                checked={*props.bind_handle}
                oninput={oninput}
                type="checkbox"
            />
            <span class="checkmark"></span>
        </label>
    }
}
