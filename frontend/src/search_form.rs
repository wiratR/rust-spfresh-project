use leptos::prelude::*;
use leptos::ev::Event;
use serde::{Serialize, Deserialize};
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Request, RequestInit, Response, console, window};
use wasm_bindgen_futures::JsFuture;

use crate::config::BACKEND_URL;

#[derive(Serialize)]
struct SearchQuery {
    query: String,
}

#[derive(Deserialize, Clone, Serialize)]
struct Review {
    review_title: String,
    review_body: String,
    product_id: String,
    review_rating: u32,
}

#[derive(Deserialize)]
struct SearchResult {
    reviews: Vec<Review>,
}

#[component]
pub fn SearchReview() -> impl IntoView {
    let query = RwSignal::new(String::new());
    let results = RwSignal::new(Vec::<Review>::new());
    let status = RwSignal::new(String::new());
    let response_text = RwSignal::new(String::new());

    let query_clone_input = query.clone();
    let on_input = move |ev: Event| {
        let input: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
        query_clone_input.set(input.value());
    };

    let query_clone = query.clone();
    let results_clone = results.clone();
    let status_clone = status.clone();
    let response_text_clone = response_text.clone();

    let on_search = move |_| {
        let q = query_clone.get();

        if q.is_empty() {
            results_clone.set(Vec::new());
            status_clone.set("กรุณากรอกคำค้นหา".to_string());
            response_text_clone.set(String::new());
            return;
        }

        status_clone.set("⏳ กำลังค้นหา...".to_string());

        spawn_local(async move {
            let body = match serde_json::to_string(&SearchQuery { query: q.clone() }) {
                Ok(b) => b,
                Err(e) => {
                    console::log_1(&format!("❌ JSON serialize error: {:?}", e).into());
                    status_clone.set("❌ JSON serialize failed".to_string());
                    return;
                }
            };

            let opts = RequestInit::new();
            opts.set_method("POST");
            opts.set_body(&JsValue::from_str(&body));

            let url = format!("{}/search", BACKEND_URL);
            console::log_1(&format!("🌐 Fetch URL: {}", url).into());

            let request = match Request::new_with_str_and_init(&url, &opts) {
                Ok(r) => r,
                Err(e) => {
                    console::log_1(&format!("❌ Request init error: {:?}", e).into());
                    status_clone.set("❌ สร้าง request ไม่สำเร็จ".to_string());
                    return;
                }
            };
            request.headers().set("Content-Type", "application/json").unwrap();

            let window = web_sys::window().unwrap();
            let resp = match JsFuture::from(window.fetch_with_request(&request)).await {
                Ok(resp) => resp,
                Err(e) => {
                    console::log_1(&format!("❌ Fetch failed: {:?}", e).into());
                    status_clone.set("❌ ไม่สามารถเชื่อม backend ได้".to_string());
                    return;
                }
            };

            let resp: Response = resp.dyn_into().unwrap();

            if resp.ok() {
                let json = JsFuture::from(resp.json().unwrap()).await.unwrap();
                let json_string = js_sys::JSON::stringify(&json).unwrap().as_string().unwrap();

                response_text_clone.set(json_string.clone());

                match serde_json::from_str::<SearchResult>(&json_string) {
                    Ok(sr) => {
                        // log reviews as JSON string
                        if let Ok(json_reviews) = serde_json::to_string(&sr.reviews) {
                            console::log_1(&JsValue::from_str(&json_reviews));
                        }
                        results_clone.set(sr.reviews.clone());
                        status_clone.set("✅ ค้นหาสำเร็จ".to_string());
                    }
                    Err(e) => {
                        console::log_1(&format!("❌ JSON parse error: {:?}", e).into());
                        status_clone.set("❌ แปลง JSON ไม่สำเร็จ".to_string());
                    }
                }
            } else {
                status_clone.set("❌ ค้นหาไม่สำเร็จ".to_string());
            }
        });
    };

    let response_text_clone_alert = response_text.clone();
    let on_show_popup = move |_| {
        let resp_text = response_text_clone_alert.get();
        if !resp_text.is_empty() {
            if let Some(win) = window() {
                let _ = win.alert_with_message(&resp_text);
            }
        }
    };

    view! {
        <form on:submit=|ev| ev.prevent_default()>
            <input
                type="text"
                placeholder="พิมพ์คำค้นหา..."
                on:input=on_input
                value=move || query.get()
            />
            <button type="button" on:click=on_search>"ค้นหา"</button>

            <p>{status.get()}</p>
            <p>"จำนวนผลลัพธ์: " {move || results.get().len()}</p>

            <ul style="list-style:none; padding:0;">
                <For
                    each=move || results.get()
                    key=|r| format!("{}-{}", r.product_id, r.review_title)
                    children=move |r| {
                        view! {
                            <li
                                style="
                                    border: 1px solid #ccc;
                                    margin: 10px 0;
                                    padding: 10px;
                                    border-radius: 8px;
                                    box-shadow: 1px 1px 3px rgba(0,0,0,0.1);
                                "
                            >
                                <b>{r.review_title.clone()}</b>
                                {" ("}{r.product_id.clone()}{") - "}
                                {format!("{}⭐", r.review_rating)}
                                <p style="margin-top:8px;">{r.review_body.clone()}</p>
                            </li>
                        }
                    }
                />
            </ul>

            <button type="button" on:click=on_show_popup style="margin-top:10px;">
                "แสดงผลค้นหา (raw JSON)"
            </button>
        </form>
    }
}
