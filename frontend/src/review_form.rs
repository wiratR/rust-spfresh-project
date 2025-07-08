// src/view_form.rs
use leptos::prelude::*;
use serde::Serialize;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{HtmlInputElement, HtmlTextAreaElement, Request, RequestInit, Response};
use web_sys::console;
use crate::config::BACKEND_URL;

#[derive(Serialize)]
struct Review {
    review_title: String,
    review_body: String,
    product_id: String,
    review_rating: u8,
}

#[component]
pub fn ReviewForm() -> impl IntoView {
    let review_title = RwSignal::new(String::new());
    let review_body = RwSignal::new(String::new());
    let product_id = RwSignal::new(String::new());
    let review_rating = RwSignal::new(5u8);
    let status = RwSignal::new(String::new());

    fn get_input_value(ev: &leptos::ev::Event) -> String {
        ev.target()
            .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
            .map(|input| input.value())
            .unwrap_or_default()
    }

    fn get_textarea_value(ev: &leptos::ev::Event) -> String {
        ev.target()
            .and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok())
            .map(|ta| ta.value())
            .unwrap_or_default()
    }

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let review = Review {
            review_title: review_title.get(),
            review_body: review_body.get(),
            product_id: product_id.get(),
            review_rating: review_rating.get(),
        };

        let status = status.clone();

        spawn_local(async move {
            let json_str = serde_json::to_string(&review).unwrap();

            // Log JSON review data to browser console
            console::log_1(&JsValue::from_str(&format!("📦 Sending review: {}", json_str)));

            let opts = RequestInit::new();
            opts.set_method("POST");
            opts.set_body(&JsValue::from_str(&json_str));

            let url = format!("{}/reviews", BACKEND_URL);
            let request = Request::new_with_str_and_init(&url, &opts).unwrap();
            request.headers().set("Content-Type", "application/json").unwrap();

            let window = web_sys::window().unwrap();
            let resp = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();
            let resp: Response = resp.dyn_into().unwrap();

            if resp.ok() {
                status.set("✅ ส่งรีวิวเรียบร้อยแล้ว".to_string());

                // Show alert popup on success
                window.alert_with_message("✅ ส่งรีวิวเรียบร้อยแล้ว").ok();
            } else {
                status.set("❌ เกิดข้อผิดพลาด".to_string());

                // Show alert popup on error
                window.alert_with_message("❌ ไม่สามารถส่งรีวิวได้").ok();
            }
        });
    };

    view! {
        <form on:submit=on_submit>
            <h2>"ส่งรีวิวสินค้า"</h2>

            <label for="title">"หัวข้อ:"</label>
            <input
                id="title"
                type="text"
                on:input=move |ev| review_title.set(get_input_value(&ev))
                prop:value=move || review_title.get()
            />

            <label for="body">"เนื้อหา:"</label>
            <textarea
                id="body"
                on:input=move |ev| review_body.set(get_textarea_value(&ev))
                prop:value=move || review_body.get()
            />

            <label for="pid">"Product ID:"</label>
            <input
                id="pid"
                type="text"
                on:input=move |ev| product_id.set(get_input_value(&ev))
                prop:value=move || product_id.get()
            />

            <label for="rating">"คะแนน (1-5):"</label>
            <input
                id="rating"
                type="number"
                min="1"
                max="5"
                on:input=move |ev| {
                    let val_str = get_input_value(&ev);
                    if let Ok(val) = val_str.parse() {
                        review_rating.set(val);
                    }
                }
                prop:value=move || review_rating.get().to_string()
            />

            <button type="submit">"ส่งรีวิว"</button>
            <p>{status}</p>
        </form>
    }
}
