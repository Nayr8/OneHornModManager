pub mod localisation;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::Promise;

pub async fn convert_promise_to_result(promise: Promise) -> Result<JsValue, JsValue> {
    let future = JsFuture::from(promise);
    let result = future.await?;

    Ok(result)
}