#[cfg(target_arch = "wasm32")]
pub const BACKEND_URL: &str = match option_env!("BACKEND_URL") {
    Some(url) => url,
    None => "http://backend:8000",
};

#[cfg(not(target_arch = "wasm32"))]
pub const BACKEND_URL: &str = "http://localhost:8000";