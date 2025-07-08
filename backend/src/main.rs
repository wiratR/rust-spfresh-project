use axum::{
    routing::post,
    extract::Json,
    response::IntoResponse,
    http::StatusCode,
    Json as AxumJson,
    Router,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::{OpenOptions, File},
    io::{BufRead, BufReader, Write},
    net::SocketAddr,
    sync::Mutex,
};
use once_cell::sync::Lazy;

use fastembed::embed;
use spfresh::search;


use tower_http::cors::{CorsLayer, Any};
use tower::ServiceBuilder;


#[derive(Deserialize, Serialize, Clone, Debug)]
struct Review {
    review_title: String,
    review_body: String,
    product_id: String,
    review_rating: u8,
}

#[derive(Deserialize)]
struct ReviewsBulk {
    reviews: Vec<Review>,
}

#[derive(Deserialize)]
struct SearchQuery {
    query: String,
}

#[derive(Serialize)]
struct SearchResult {
    reviews: Vec<Review>,
}

type EmbeddingVector = Vec<f32>;

static VECTOR_FILE: Lazy<Mutex<File>> = Lazy::new(|| {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open("data/reviews.index")
        .expect("Cannot open vector index file")
        .into()
});

static METADATA_FILE: Lazy<Mutex<File>> = Lazy::new(|| {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open("data/reviews.jsonl")
        .expect("Cannot open metadata file")
        .into()
});

fn append_vector(vec: &EmbeddingVector) -> std::io::Result<()> {
    let mut file = VECTOR_FILE.lock().unwrap();
    for v in vec {
        file.write_all(&v.to_le_bytes())?;
    }
    file.flush()?;
    Ok(())
}

fn append_metadata(review: &Review) -> std::io::Result<()> {
    let mut file = METADATA_FILE.lock().unwrap();
    let json_line = serde_json::to_string(review)?;
    file.write_all(json_line.as_bytes())?;
    file.write_all(b"\n")?;
    file.flush()?;
    Ok(())
}

async fn insert_review(Json(review): Json<Review>) -> impl IntoResponse {
    println!("Received insert_review request: {:?}", review);

    let combined = format!("{} {}", review.review_title, review.review_body);
    let embedding = embed(&combined);

    if let Err(e) = append_vector(&embedding) {
        eprintln!("Vector write error: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("Vector write error: {}", e));
    }

    if let Err(e) = append_metadata(&review) {
        eprintln!("Metadata write error: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("Metadata write error: {}", e));
    }

    (StatusCode::OK, "Review inserted".to_string())
}

async fn insert_bulk_reviews(Json(payload): Json<ReviewsBulk>) -> impl IntoResponse {
    println!("Received bulk insert request with {} reviews", payload.reviews.len());

    for (i, review) in payload.reviews.iter().enumerate() {
        println!("Processing review #{}: {:?}", i, review);

        let combined = format!("{} {}", review.review_title, review.review_body);
        let embedding = embed(&combined);

        if let Err(e) = append_vector(&embedding) {
            eprintln!("Vector write error: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("Vector write error: {}", e));
        }

        if let Err(e) = append_metadata(review) {
            eprintln!("Metadata write error: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("Metadata write error: {}", e));
        }
    }

    (StatusCode::OK, "Bulk reviews inserted".to_string())
}

async fn search_reviews(Json(query): Json<SearchQuery>) -> Result<impl IntoResponse, (StatusCode, String)> {
    println!("Received search query: {:?}", query.query);

    let q_embedding = embed(&query.query);

    let matched_indices = search(&q_embedding);

    let file = match File::open("data/reviews.jsonl") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Cannot open metadata file: {}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Cannot open metadata file: {}", e)));
        }
    };
    let reader = BufReader::new(file);

    let mut results = vec![];

    for (i, line) in reader.lines().enumerate() {
        if matched_indices.contains(&i) {
            if let Ok(json_str) = line {
                if let Ok(review) = serde_json::from_str::<Review>(&json_str) {
                    results.push(review);
                } else {
                    eprintln!("Failed to parse review at line {}", i);
                }
            }
        }
    }

    Ok(AxumJson(SearchResult { reviews: results }))
}

#[tokio::main]
async fn main() {
    
    println!("🚀 Starting backend server...");

    // // สร้าง CORS middleware ที่อนุญาต Origin
    //     let cors = CorsLayer::new()
    //     .allow_origin(AllowOrigin::exact("http://localhost:3000".parse().unwrap())) // อนุญาตจาก localhost:3000
    //     .allow_methods([Method::GET, Method::POST, Method::OPTIONS])  // อนุญาต methods ที่จะใช้
    //     .allow_headers(Any);  // อนุญาต headers ใด ๆ

    // let app = Router::new()
    //     .route("/reviews", post(insert_review))
    //     .route("/reviews/bulk", post(insert_bulk_reviews))
    //     .route("/search", post(search_reviews))
    //     .layer(cors);  // วาง CorsLayer ที่นี่ เพื่อให้ทุก route ใช้งานได้

    // axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();


    let cors = CorsLayer::new()
        .allow_origin(Any)  // หรือระบุ origin ที่ต้องการ
        .allow_methods(Any)
        // .allow_origin(AllowOrigin::exact("http://localhost:3000".parse().unwrap()))
        // .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    let app = Router::new()
        .route("/reviews", post(insert_review))
        .route("/reviews/bulk", post(insert_bulk_reviews))
        .route("/search", post(search_reviews))
        .layer(
            ServiceBuilder::new()
                .layer(cors)
        );

    // axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("🚀 Backend listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service()) // เรียกตรงนี้ได้เลย
        .await
        .unwrap();
}
