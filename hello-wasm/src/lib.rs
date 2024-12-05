use wasm_bindgen::prelude::*;
use gloo_timers::future::sleep;
use std::time::Duration;
use csv::ReaderBuilder;
use serde_json::{Value as JsonValue, json};

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

#[wasm_bindgen]
pub async fn csv_to_json(csv_data: &[u8]) -> Result<JsValue, JsValue> {
    // Create a CSV reader with semicolon delimiter
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .trim(csv::Trim::All)  // Trim whitespace from fields
        .from_reader(csv_data);
    
    // Get headers
    let headers = reader.headers()
        .map_err(|e| JsValue::from_str(&format!("Failed to read headers: {}", e)))?
        .clone();
    
    // Convert records to JSON
    let mut json_array = Vec::new();
    
    for result in reader.records() {
        let record = result.map_err(|e| JsValue::from_str(&format!("Failed to read record: {}", e)))?;
        let mut json_record = serde_json::Map::new();
        
        for (i, field) in record.iter().enumerate() {
            if i < headers.len() {                
                json_record.insert(
                    headers[i].trim().to_string(),
                    JsonValue::String(field.trim().to_string())
                );
            }
        }
        
        json_array.push(JsonValue::Object(json_record));
    }
    
    // Convert to JsValue
    let json_string = serde_json::to_string(&json_array)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize JSON: {}", e)))?;
    
    Ok(JsValue::from_str(&json_string))
}


