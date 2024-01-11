use super::Chart;

use gloo::timers::callback::Timeout;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub config: String,
}

pub struct ChartJs {
    pub chart: Chart,
    pub draw_timer: Timeout,
    pub chart_name: String,
}

impl Component for ChartJs {
    type Message = ();
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {
        let stand_alone_timer = {
            let link = ctx.link().clone();
            Timeout::new(10, move || link.send_message(()))
        };
        Self {
            chart_name: format!("chart-{}", crate::generate_random_string(10)),
            chart: Chart::new(),
            draw_timer: stand_alone_timer,
        }
    }
    fn update(&mut self, ctx: &Context<Self>, _msg: Self::Message) -> bool {
        self.chart.draw(&self.chart_name, &ctx.props().config);
        true
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <canvas id={self.chart_name.clone()}></canvas>
        }
    }
}
