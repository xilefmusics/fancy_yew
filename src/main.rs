use yew::prelude::*;

mod fancy_yew;

pub use fancy_yew::ChartJs;

#[function_component]
fn App() -> Html {
    html! {
        <ChartJs/>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
