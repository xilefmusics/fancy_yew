use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/components/chart_js/chart.js")]
extern "C" {
    pub type MyChart;
    #[wasm_bindgen(constructor)]
    pub fn new() -> MyChart;
    #[wasm_bindgen(method)]
    pub fn draw(this: &MyChart, element_id: &str, config: &str);
}
