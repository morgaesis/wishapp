use lambda_http::{Body, Error, Request, Response};
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::sync::Mutex;
use once_cell::sync::Lazy;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Wishlist {
    pub id: String,
    pub items: Vec<String>,
    pub owner: String,
}

pub static WISHLISTS: Lazy<Mutex<Vec<Wishlist>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub async fn handle_get(event: Request) -> Result<Response<Body>, Error> {
    match event.uri().path() {
        "/health" => Ok(Response::builder().status(200).body("OK".into())?),
        "/wishlists" => {
            let wishlists = WISHLISTS.lock().unwrap();
            Ok(Response::builder()
                .status(200)
                .body(serde_json::to_string(&*wishlists)?.into())?)
        }
        _ => Ok(Response::builder().status(404).body("Not Found".into())?),
    }
}

pub async fn handle_post(event: Request) -> Result<Response<Body>, Error> {
    let wishlist: Wishlist = serde_json::from_slice(event.body().as_ref())?;
    WISHLISTS.lock().unwrap().push(wishlist.clone());
    Ok(Response::builder()
        .status(201)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&wishlist)?.into())?)
}

pub async fn handle_put(event: Request) -> Result<Response<Body>, Error> {
    let updated: Wishlist = serde_json::from_slice(event.body().as_ref())?;
    let mut wishlists = WISHLISTS.lock().unwrap();
    if let Some(pos) = wishlists.iter().position(|w| w.id == updated.id) {
        wishlists[pos] = updated.clone();
        Ok(Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&updated)?.into())?)
    } else {
        Ok(Response::builder()
            .status(404)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&json!({"error": "Not Found"}))?.into())?)
    }
}

pub async fn handle_delete(event: Request) -> Result<Response<Body>, Error> {
    println!("Deleting wishlist with ID: {}", event.uri().path()); println!("Delete request URI: {}", event.uri()); let id = event.uri().path().split('/').last().unwrap_or(""); println!("Extracted ID: {}", id);
    let mut wishlists = WISHLISTS.lock().unwrap();
    if let Some(pos) = wishlists.iter().position(|w| w.id == id) {
        wishlists.remove(pos);
        Ok(Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&json!({"status": "Deleted"}))?.into())?)
    } else {
        Ok(Response::builder()
            .status(404)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&json!({"error": "Not Found"}))?.into())?)
    }
}
