use yew::prelude::*;

use fancy_yew::components::ChartJs;
use stylist::{css, yew::Global};

#[function_component]
fn App() -> Html {
    html! {
        <>
            <Global css={css!("html,body{padding: 0;margin: 0;border: 0;background: #1e1e1e; overflow: hidden; overscroll-behavior: none; height: 100dvh; }")} />
            <ChartJs/>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
