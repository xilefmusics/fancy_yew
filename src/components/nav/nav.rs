use stylist::Style;
use yew::prelude::*;

#[function_component]
pub fn Nav() -> Html {
    let mobile = false;
    let open = use_state(|| !mobile);
    let mode = if mobile {
        if *open {
            "small-open"
        } else {
            "small-closed"
        }
    } else {
        if *open {
            "wide-open"
        } else {
            "wide-closed"
        }
    };
    let toggle_open = move |_: MouseEvent| {
        open.set(!*open);
    };

    html! {
        <div
            id="nav"
            class={Style::new(include_str!("nav.css")).expect("Unwrapping CSS should work!")}
        >
            <ul class={mode}>
                <li class="menu">
                    <a onclick={toggle_open}>
                        <span class="material-symbols-outlined icon">{if mobile {"close"} else {"menu"}}</span>
                        <span class="text"></span>
                    </a>
                </li>
                <li>
                    <a>
                        <span class="material-symbols-outlined icon">{"menu_book"}</span>
                        <span class="text">{"Dashboard"}</span>
                    </a>
                </li>
                <li class="selected">
                    <a>
                        <span class="material-symbols-outlined icon">{"menu_book"}</span>
                        <span class="text">{"Transactions"}</span>
                    </a>
                </li>
                <li>
                    <a>
                        <span class="material-symbols-outlined icon">{"menu_book"}</span>
                        <span class="text">{"Pods"}</span>
                    </a>
                </li>
                <li>
                    <a>
                        <span class="material-symbols-outlined icon">{"menu_book"}</span>
                        <span class="text">{"Budgets"}</span>
                    </a>
                </li>
                <li>
                    <a>
                        <span class="material-symbols-outlined icon">{"menu_book"}</span>
                        <span class="text">{"Debts"}</span>
                    </a>
                </li>
                <li>
                    <a>
                        <span class="material-symbols-outlined icon">{"menu_book"}</span>
                        <span class="text">{"Contracts"}</span>
                    </a>
                </li>
            </ul>
        </div>
    }
}
