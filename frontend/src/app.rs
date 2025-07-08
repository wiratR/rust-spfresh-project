// src/app.rs
use leptos::prelude::*;
use crate::review_form::ReviewForm;
use crate::search_form::SearchReview;
use crate::bulk_form::BulkReviewForm;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main>
            <h1>"ระบบรีวิว"</h1>
            <ReviewForm />

            <BulkReviewForm /> 

            <SearchReview />
        </main>
    }
}

