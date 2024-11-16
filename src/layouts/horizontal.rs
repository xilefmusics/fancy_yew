use super::Navable;

use stylist::yew::Global;
use yew::prelude::*;
use yew_hooks::use_window_size;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props<R: Routable + Navable> {
    pub children: Html,
    pub nav_routes: Vec<R>,
    #[prop_or_default]
    pub account_route: Option<R>,
    #[prop_or_default]
    pub fullscreen: bool,
}

#[function_component(HorizontalLayout)]
pub fn horizontal_layout<R: Routable + Navable>(props: &Props<R>) -> Html {
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

    let account_route = props
        .account_route
        .clone()
        .map(|route| route.to_nav_item().build(&navigator, location.path()))
        .unwrap_or(html! {
            <li><span class="material-symbols-outlined account icon">
                {"account_circle"}
            </span></li>
        });

    let closed = use_state(|| true);
    let toggle_closed = {
        let closed = closed.clone();
        move |_: MouseEvent| {
            closed.set(!*closed);
        }
    };
    let (width, height) = use_window_size();
    let mobile = height > width;

    html! {
        <>
            <Global css={include_str!("horizontal.css")} />
            if !props.fullscreen {
                <header id="horizontal-layout" class={if mobile {"mobile"} else {""}}>
                    <li><span
                        class="material-symbols-outlined icon"
                        onclick={toggle_closed.clone()}
                    >
                        {"menu"}
                    </span></li>
                    <div class="header-main"></div>
                    {
                        account_route
                    }
                </header>
                <aside id="horizontal-layout" class={
                    if *closed && mobile {"closed mobile"}
                    else if *closed {"closed"}
                    else if mobile {"mobile"} else {""}
                }>
                    <nav id="horizontal-layout">
                        <ul>
                            <li class="close"><span
                                class="material-symbols-outlined icon"
                                onclick={toggle_closed}
                            >
                                {"close"}
                            </span></li>
                            {
                                nav_items
                            }
                        </ul>
                    </nav>
                </aside>
            }
            <main id="horizontal-layout" class={
                if mobile {"mobile"}
                else if props.fullscreen {"fullscreen"}
                else {""}
            }>
                {
                    props.children.clone()
                }
            </main>
        </>
    }
}
