use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn pipe(json: &str) -> Result<String, wasm_bindgen::JsError> {
    Ok(serde_json_canonicalizer::pipe(json)?)
}
