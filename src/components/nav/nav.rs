use stylist::Style;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
    pub mobile: bool,
    pub open: bool,
    pub toggle_open: Callback<()>,
}

#[function_component]
pub fn Nav(props: &Props) -> Html {
    let mode = if props.mobile {
        if props.open {
            "small-open"
        } else {
            "small-closed"
        }
    } else {
        if props.open {
            "wide-open"
        } else {
            "wide-closed"
        }
    };

    let toggle_open = {
        let toggle_open = props.toggle_open.clone();
        move |_: MouseEvent| toggle_open.emit(())
    };

    html! {
        <div
            id="nav"
            class={Style::new(include_str!("nav.css")).expect("Unwrapping CSS should work!")}
        >
            <ul class={mode}>
                <li class="menu">
                    <a onclick={toggle_open}>
                        <span class="material-symbols-outlined icon">{if props.mobile {"close"} else {"menu"}}</span>
                        <span class="text"></span>
                    </a>
                </li>
                {props.children.clone()}
            </ul>
        </div>
    }
}
