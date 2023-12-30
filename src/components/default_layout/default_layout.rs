use crate::components::{Nav, TopBar};

use stylist::Style;
use yew::prelude::*;

#[function_component]
pub fn DefaultLayout() -> Html {
    let mobile = use_state(|| true);
    let toggle_mobile = {
        let mobile = mobile.clone();
        move |_: MouseEvent| {
            mobile.set(!*mobile);
        }
    };

    let menu_open = use_state(|| false);
    let toggle_menu_open = {
        let menu_open = menu_open.clone();
        Callback::from(move |()| {
            menu_open.set(!*menu_open);
        })
    };

    html! {
        <div class={Style::new(include_str!("default_layout.css")).expect("Unwrapping CSS should work!")}>
            <div id="left">
                <Nav
                    mobile={*mobile}
                    open={*menu_open}
                    toggle_open={toggle_menu_open.clone()}
                />
            </div>
            <div id="right">
                <div id="top">
                    <TopBar
                        mobile={*mobile}
                        menu_open={*menu_open}
                        toggle_menu_open={toggle_menu_open}
                    />
                </div>
                <div id="center">
                </div>
            </div>
        </div>
    }
}
