use super::ChartJsWrapper;

use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement, HtmlScriptElement, Window};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub config: String,
}

pub struct Chart {
    pub chart_name: String,
    pub chart: Option<ChartJsWrapper>,
}

impl Component for Chart {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            chart_name: format!("chart-{}", crate::generate_random_string(10)),
            chart: None,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let window: Window = web_sys::window().expect("should have a window in this context");
            let document: Document = window.document().expect("should have a document on window");

            if document
                .query_selector("script[data-id='chart-js-script']")
                .unwrap()
                .is_none()
            {
                let script = document
                    .create_element("script")
                    .expect("should be able to create script element");
                script
                    .set_attribute("data-id", "chart-js-script")
                    .expect("should be able to set data-id attribute");
                script.set_inner_html(include_str!("external_dependencies/chartjs_4.4.4.js"));
                let script: HtmlScriptElement = script.dyn_into::<HtmlScriptElement>().unwrap();

                let body: HtmlElement = document.body().expect("document should have a body");
                body.append_child(&script)
                    .expect("should be able to append script to body");
            }

            let chart = ChartJsWrapper::new();
            chart.draw(&self.chart_name, &ctx.props().config);
            self.chart = Some(chart);
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <canvas id={self.chart_name.clone()}></canvas>
        }
    }
}
