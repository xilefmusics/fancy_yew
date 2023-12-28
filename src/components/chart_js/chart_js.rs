use super::{Chart, ConfigBuilder};

use gloo::timers::callback::Timeout;
use yew::prelude::*;

pub enum Msg {
    Draw,
    DoNothing,
}
pub struct ChartJs {
    pub chart: Chart,
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
            chart: Chart::new(),
            draw_timer: stand_alone_timer,
            input_ref: NodeRef::default(),
            config: ConfigBuilder::new().build().unwrap(),
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
