use super::{Config, Data, Dataset};

use gloo::timers::callback::Timeout;
use serde_json;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/src/components/chart_js/chart.js")]
extern "C" {
    pub type MyChart;
    #[wasm_bindgen(constructor)]
    pub fn new() -> MyChart;
    #[wasm_bindgen(method)]
    pub fn draw(this: &MyChart, element_id: &str, config: &str);
}

pub enum Msg {
    Draw,
    DoNothing,
}
pub struct ChartJs {
    pub chart: MyChart,
    pub input_ref: NodeRef,
    pub draw_timer: Timeout,
    pub config: String,
}
impl Component for ChartJs {
    type Message = Msg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link();
        let stand_alone_timer = {
            let link = link.clone();
            Timeout::new(10, move || link.send_message(Msg::Draw))
        };
        Self {
            chart: MyChart::new(),
            draw_timer: stand_alone_timer,
            input_ref: NodeRef::default(),
            config: serde_json::to_string(&Config {
                chart_type: Some("line"),
                data: Some(&Data {
                    labels: &vec![1, 2, 3, 4, 5, 6],
                    datasets: &vec![&Dataset {
                        label: "Widget Data",
                        data: &vec![10, 35, 30, 20, 100, 15],
                    }],
                }),
                options: None,
            })
            .unwrap(),
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Draw => {
                self.chart.draw("myChart", &self.config);
                true
            }
            Msg::DoNothing => true,
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <section class="section">
            <div class="container">
                <canvas id="myChart" width="1200" height="500"></canvas>
            </div>
            </section>
        }
    }
}
