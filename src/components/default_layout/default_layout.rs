use crate::components::{Nav, Navable, TopBar};

use gloo::utils::window;
use stylist::yew::Global;
use stylist::Style;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props<R: Routable + Navable> {
    pub children: Html,
    pub nav_routes: Vec<R>,
}

#[function_component]
pub fn DefaultLayout<R: Routable + Navable>(props: &Props<R>) -> Html {
    let mobile = use_state(|| true);
    let first_render = use_state(|| true);
    {
        let mobile = mobile.clone();
        let first_render = first_render.clone();
        use_effect(move || {
            if !*first_render {
                return;
            }
            first_render.set(false);
            let width = window().inner_width().unwrap().as_f64();
            let height = window().inner_height().unwrap().as_f64();
            mobile.set(height > width);
        })
    }

    let menu_open = use_state(|| false);
    let toggle_menu_open = {
        let menu_open = menu_open.clone();
        Callback::from(move |()| {
            menu_open.set(!*menu_open);
        })
    };
    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();
    let nav_items = props
        .nav_routes
        .iter()
        .map(|route| {
            route
                .clone()
                .to_nav_item()
                .build(&navigator, location.path())
        })
        .collect::<Html>();

    html! {
        <>
            <Global css={include_str!("main.css")} />
            <div class={Style::new(include_str!("default_layout.css")).expect("Unwrapping CSS should work!")}>
                <div id="left" class={if *mobile {"mobile"} else {""}}>
                    <Nav
                        mobile={*mobile}
                        open={*menu_open}
                        toggle_open={toggle_menu_open.clone()}
                    >
                        {nav_items}
                    </Nav>
                </div>
                <div id="right" class={if *mobile {"mobile"} else {""}}>
                    <div id="top">
                        <TopBar
                            mobile={*mobile}
                            menu_open={*menu_open}
                            toggle_menu_open={toggle_menu_open}
                        />
                    </div>
                    <div id="center">
                        {
                            props.children.clone()
                        }
                    </div>
                </div>
            </div>
            </>
    }
}
