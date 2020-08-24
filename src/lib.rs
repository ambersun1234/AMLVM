use wasm_bindgen::prelude::*;
use kmeans;
use imagenet;

#[wasm_bindgen]
pub fn wasm_hello(s: &str) -> String {
    let r1 = String::from("Hello, ");
    let r2 = String::from(". Send from Rust");
    return r1 + s + &r2;
}

#[wasm_bindgen]
pub fn wasm_infer(model_data: &[u8], image_data: &[u8], image_height: i32, image_width: i32) -> String {
    return imagenet::infer(model_data, image_data, image_height, image_width)
}
