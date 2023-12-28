use yew::prelude::*;

use fancy_yew::components::ChartJs;

#[function_component]
fn App() -> Html {
    html! {
        <ChartJs/>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
