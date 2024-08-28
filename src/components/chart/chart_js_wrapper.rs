use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/components/chart/chart_js_wrapper.js")]
extern "C" {
    pub type ChartJsWrapper;
    #[wasm_bindgen(constructor)]
    pub fn new() -> ChartJsWrapper;
    #[wasm_bindgen(method)]
    pub fn draw(this: &ChartJsWrapper, element_id: &str, config: &str);
}
