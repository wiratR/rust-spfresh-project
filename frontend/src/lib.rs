pub mod app;
pub mod config;
pub mod review_form;
pub mod search_form;
pub mod bulk_form;


#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::App; // <- ต้อง import App เข้ามา
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}