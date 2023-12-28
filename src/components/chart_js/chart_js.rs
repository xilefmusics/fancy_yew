use gloo::timers::callback::Timeout;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/src/components/chart_js/my_chart.js")]
extern "C" {
    pub type MyChart;
    #[wasm_bindgen(constructor)]
    pub fn new() -> MyChart;
    #[wasm_bindgen(method)]
    pub fn draw(this: &MyChart, element_id: &str);
}

pub enum Msg {
    Draw,
    DoNothing,
}
pub struct ChartJs {
    pub chart: MyChart,
    pub input_ref: NodeRef,
    pub draw_timer: Timeout,
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
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Draw => {
                self.chart.draw("myChart");
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
