// Fix for now: https://github.com/rustwasm/wasm-bindgen/issues/2774
#![allow(clippy::unused_unit)]

pub mod app;
pub mod routes;

use wasm_bindgen::prelude::*;
use app::App;

// Use `std::alloc` as the global allocator.
#[global_allocator]
static ALLOC: std::alloc::System = std::alloc::System;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
    Ok(())
}

pub fn main() {
    run_app().expect("Ok");
}