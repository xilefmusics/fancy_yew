use yew::prelude::*;

use fancy_yew::components::{ChartJs, ConfigBuilder, DefaultLayout, Nav, TopBar};
use stylist::{yew::Global, Style};

#[function_component]
fn App() -> Html {
    // let chart_config = ConfigBuilder::bar()
    //     .labels(&vec!["Jan", "Feb", "Mar", "Apr", "Mai", "Jun"])
    //     .dataset(&vec![1, 6, 3, 4, 2, 5])
    //     .dataset_label("First Data")
    //     .dataset_stack(0)
    //     .dataset_border_color_rgba(0, 255, 0, 0.4)
    //     .dataset_border_width(1)
    //     .dataset_background_color_rgba(0, 255, 0, 0.2)
    //     .dataset(&vec![6, 3, 4, 2, 5, 1])
    //     .dataset_label("Second Data")
    //     .dataset_stack(0)
    //     .dataset_border_color_rgba(0, 0, 255, 0.5)
    //     .dataset_border_width(1)
    //     .dataset_background_color_rgba(0, 0, 255, 0.2)
    //     .dataset(&vec![3, 4, 2, 5, 1, 6])
    //     .dataset_label("Third Data")
    //     .dataset_border_color_rgba(255, 0, 0, 0.45)
    //     .dataset_border_width(1)
    //     .dataset_background_color_rgba(255, 0, 0, 0.2)
    //     .build()
    //     .unwrap();
    // html! {
    //     <>
    //         <Global css={css!("html,body{padding: 0;margin: 0;border: 0;background: #1e1e1e; overflow: hidden; overscroll-behavior: none; height: 100dvh; font-family: 'Ubuntu', sans-serif; }")} />
    //         <div style={"display: flex; column; height: 100%; width: 100%"}>
    //             <Nav />
    //             <div style={"flex: 1; height: 100%; width: 100%"}>
    //                 <TopBar />
    //                 <ChartJs config={chart_config}/>
    //             </div>
    //         </div>
    //     </>
    // }
    html! {
        <>
            <Global css={include_str!("main.css")} />
            <DefaultLayout />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
