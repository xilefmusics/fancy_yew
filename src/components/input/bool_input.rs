use super::StringInput;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub bind_handle: UseStateHandle<bool>,
}

#[function_component(BoolInput)]
pub fn bool_input(props: &Props) -> Html {
    let value = if *props.bind_handle {
        "True".into()
    } else {
        "False".into()
    };
    let value = use_state(|| value);
    let callback = {
        let handle = props.bind_handle.clone();
        Callback::from(move |value: String| handle.set(value == "True"))
    };
    html! {
        <StringInput
            bind_handle={value}
            options={vec!["True".into(), "False".into()]}
            strict=true
            callback={callback}
            />
    }
}
