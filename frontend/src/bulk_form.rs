// src/bulk_form.rs
use leptos::prelude::*;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use web_sys::RequestInit;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, Response, console};
use crate::config::BACKEND_URL;

#[derive(Serialize, Clone)]
struct Review {
    review_title: String,
    review_body: String,
    product_id: String,
    review_rating: u8,
}

#[component]
pub fn BulkReviewForm() -> impl IntoView {
    let status = RwSignal::new(String::new());
    let reviews = RwSignal::new(vec![
        Review {
            review_title: "Title A".into(),
            review_body: "Body A".into(),
            product_id: "P001".into(),
            review_rating: 4,
        },
        Review {
            review_title: "Title B".into(),
            review_body: "Body B".into(),
            product_id: "P002".into(),
            review_rating: 5,
        },
    ]);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let data = serde_json::json!({ "reviews": reviews.get() });
        let status = status.clone();

        spawn_local(async move {
            let body_str = serde_json::to_string(&data).unwrap();

            // Log JSON data to console
            console::log_1(&JsValue::from_str(&format!("📦 Sending bulk reviews: {}", body_str)));

            let opts = RequestInit::new();

            opts.set_method("POST");
            opts.set_body(&JsValue::from_str(&body_str));

            let url = format!("{}/reviews/bulk", BACKEND_URL);
            let request = Request::new_with_str_and_init(&url, &opts).unwrap();
            request.headers().set("Content-Type", "application/json").unwrap();

            let window = web_sys::window().unwrap();
            let resp = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();
            let resp: Response = resp.dyn_into().unwrap();

            if resp.ok() {
                status.set("✅ ส่งรีวิวชุดใหญ่เรียบร้อยแล้ว".to_string());
                window.alert_with_message("✅ ส่งรีวิวชุดใหญ่เรียบร้อยแล้ว").ok();
            } else {
                status.set("❌ ส่งรีวิวชุดใหญ่ไม่สำเร็จ".to_string());
                window.alert_with_message("❌ ส่งรีวิวชุดใหญ่ไม่สำเร็จ").ok();
            }
        });
    };

    view! {
        <form on:submit=on_submit>
            <h2>"ส่งหลายรีวิว (Bulk)"</h2>
            <button type="submit">"ส่งทั้งหมด"</button>
            <p>{status}</p>
        </form>
    }
}