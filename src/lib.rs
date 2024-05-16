pub mod app;
mod components;
mod error_template;
pub mod errors;
#[cfg(feature = "ssr")]
pub mod fileserv;
mod layouts;
mod models;
mod routes;
#[cfg(feature = "ssr")]
pub mod state;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
