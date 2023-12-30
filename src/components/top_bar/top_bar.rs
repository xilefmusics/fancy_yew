use stylist::Style;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub mobile: bool,
    pub menu_open: bool,
    pub toggle_menu_open: Callback<()>,
}

#[function_component]
pub fn TopBar(props: &Props) -> Html {
    let toggle_menu_open = {
        let toggle_menu_open = props.toggle_menu_open.clone();
        move |_: MouseEvent| toggle_menu_open.emit(())
    };

    html! {
        <div class={Style::new(include_str!("top_bar.css")).expect("Unwrapping CSS should work!")} >
            <div id="bar" class={if props.mobile {"mobile"} else {""}}>
                <div id="left">
                    {if props.mobile && ! props.menu_open {
                        html!{
                            <span
                                class="material-symbols-outlined icon"
                                onclick={toggle_menu_open}
                            >
                                {"menu"}
                            </span>
                        }
                    }else {
                        html!{}
                    }}
                </div>
                <div id="middle">
                </div>
                <div id="right">
                </div>
            </div>
        </div>
    }
}
