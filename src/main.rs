use yew::prelude::*;

use fancy_yew::components::{ChartJs, ConfigBuilder, DefaultLayout};
use stylist::yew::Global;

#[function_component]
fn App() -> Html {
    let chart_config = ConfigBuilder::bar()
        .labels(&vec!["Jan", "Feb", "Mar", "Apr", "Mai", "Jun"])
        .dataset(&vec![1, 6, 3, 4, 2, 5])
        .dataset_label("First Data")
        .dataset_stack(0)
        .dataset_border_color_rgba(0, 255, 0, 0.4)
        .dataset_border_width(1)
        .dataset_background_color_rgba(0, 255, 0, 0.2)
        .dataset(&vec![6, 3, 4, 2, 5, 1])
        .dataset_label("Second Data")
        .dataset_stack(0)
        .dataset_border_color_rgba(0, 0, 255, 0.5)
        .dataset_border_width(1)
        .dataset_background_color_rgba(0, 0, 255, 0.2)
        .dataset(&vec![3, 4, 2, 5, 1, 6])
        .dataset_label("Third Data")
        .dataset_border_color_rgba(255, 0, 0, 0.45)
        .dataset_border_width(1)
        .dataset_background_color_rgba(255, 0, 0, 0.2)
        .build()
        .unwrap();
    html! {
        <>
            <Global css={include_str!("main.css")} />
            <DefaultLayout>
                <ChartJs config={chart_config}/>
            </DefaultLayout>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
