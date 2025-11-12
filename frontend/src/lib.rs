pub mod app;

use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn run_app() {
    mount_to_body(app::App);
}
