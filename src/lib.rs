use lambda_http::{Body, Error, Request, Response};
use once_cell::sync::Lazy;
use serde_json::{from_slice, json};
use std::sync::Mutex;

pub mod handlers {
    pub mod wishlist;
    pub use wishlist::Wishlist;
}

pub static WISHLISTS: Lazy<Mutex<Vec<handlers::wishlist::Wishlist>>> =
    Lazy::new(|| Mutex::new(Vec::new()));

pub async fn handle_get(_event: Request) -> Result<Response<Body>, Error> {
    let wishlists = WISHLISTS.lock().unwrap();
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(json!(*wishlists).to_string().into())?)
}

pub async fn handle_post(event: Request) -> Result<Response<Body>, Error> {
    let body = event.body();
    let mut wishlist: handlers::wishlist::Wishlist = from_slice(body)?;

    if wishlist.name.is_empty() {
        return Ok(Response::builder()
            .status(400)
            .body("Name cannot be empty".into())?);
    }

    // Add default chocolate item if not already present
    if !wishlist.items.contains(&"Chocolate".to_string()) {
        wishlist.items.push("Chocolate".to_string());
    }

    let mut wishlists = WISHLISTS.lock().unwrap();
    wishlists.push(wishlist);
    let last = wishlists.last().unwrap();
    Ok(Response::builder()
        .status(201)
        .header("Content-Type", "application/json")
        .body(json!(last).to_string().into())?)
}

pub async fn handle_put(event: Request) -> Result<Response<Body>, Error> {
    let body = event.body();
    let updated: handlers::wishlist::Wishlist = from_slice(body)?;

    if updated.id.is_empty() {
        return Ok(Response::builder()
            .status(400)
            .body("ID cannot be empty".into())?);
    }

    let mut wishlists = WISHLISTS.lock().unwrap();
    if let Some(pos) = wishlists.iter().position(|w| w.id == updated.id) {
        wishlists[pos] = updated;
        Ok(Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(json!(wishlists[pos]).to_string().into())?)
    } else {
        Ok(Response::builder()
            .status(404)
            .body("Wishlist not found".into())?)
    }
}
pub async fn handle_delete(event: Request) -> Result<Response<Body>, Error> {
    let body = event.body();
    let data: serde_json::Value = from_slice(body)?;

    // Handle clear_all request
    if data.get("clear_all").is_some() {
        let mut wishlists = WISHLISTS.lock().unwrap();
        wishlists.clear();
        return Ok(Response::builder().status(204).body("".into())?);
    }

    // Normal deletion by ID
    let id = data["id"].as_str().unwrap_or("");
    let mut wishlists = WISHLISTS.lock().unwrap();
    if let Some(pos) = wishlists.iter().position(|w| w.id == id) {
        wishlists.remove(pos);
        Ok(Response::builder().status(204).body("".into())?)
    } else {
        Ok(Response::builder()
            .status(404)
            .body("Wishlist not found".into())?)
    }
}
