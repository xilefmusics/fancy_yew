use super::{Chart, ConfigBuilder};

use gloo::timers::callback::Timeout;
use yew::prelude::*;

pub struct ChartJs {
    pub chart: Chart,
    pub draw_timer: Timeout,
    pub config: String,
}

impl Component for ChartJs {
    type Message = ();
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        let stand_alone_timer = {
            let link = ctx.link().clone();
            Timeout::new(10, move || link.send_message(()))
        };
        Self {
            chart: Chart::new(),
            draw_timer: stand_alone_timer,
            config: ConfigBuilder::new().build().unwrap(),
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        self.chart.draw("chart", &self.config);
        true
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <canvas id="chart"></canvas>
        }
    }
}
