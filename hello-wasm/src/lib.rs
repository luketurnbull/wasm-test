use wasm_bindgen::prelude::*;
use gloo_timers::future::sleep;
use std::time::Duration;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub fn bare_bones() {
    log("Hello from Rust!");
    log_u32(42);
    log_many("Logging", "many values!");
}

#[wasm_bindgen]
pub async fn delayed_process(input: &[u8]) -> Result<JsValue, JsValue> {
    // Simulate processing with a delay using gloo-timers
    sleep(Duration::from_millis(5000)).await;
    
    // Process the input (example: convert to base64)
    let processed_data = base64::encode(input);
    
    // Return as JsValue
    Ok(JsValue::from_str(&processed_data))
}
