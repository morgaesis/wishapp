pub mod wishlist;

pub use crate::handlers::wishlist::Wishlist;
use lambda_http::{Body, Error, Request, Response};
use once_cell::sync::Lazy;
use serde_json::json;
use std::sync::Mutex;

pub static WISHLISTS: Lazy<Mutex<Vec<Wishlist>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub async fn handle_get(event: Request) -> Result<Response<Body>, Error> {
    let path = event.uri().path();
    let cleaned_path = path.trim_start_matches("/prod"); // Remove /prod prefix
    println!("[DEBUG] GET request path: {}", path);
    println!("[DEBUG] Cleaned GET request path: {}", cleaned_path);
    match cleaned_path {
        "/health" => Ok(Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&json!({"status": "OK"}))?.into())?),
        "/wishlists" | "/wishlist" => {
            let wishlists = WISHLISTS.lock().unwrap();
            Ok(Response::builder()
                .status(200)
                .body(serde_json::to_string(&*wishlists)?.into())?)
        }
        path if path.starts_with("/wishlists/") => {
            let id = path.trim_start_matches("/wishlists/").trim_end_matches('/');
            let wishlists = WISHLISTS.lock().unwrap();
            if let Some(wishlist) = wishlists.iter().find(|w| w.id == id) {
                Ok(Response::builder()
                    .status(200)
                    .body(serde_json::to_string(wishlist)?.into())?)
            } else {
                Ok(Response::builder().status(404).body("Not Found".into())?)
            }
        }
        _ => Ok(Response::builder().status(404).body("Not Found".into())?),
    }
}

pub async fn handle_post(event: Request) -> Result<Response<Body>, Error> {
    let body = event.body().as_ref();
    println!("[DEBUG] POST body: {:?}", String::from_utf8_lossy(body));
    let wishlist: Wishlist = serde_json::from_slice(body)?;
    println!("[DEBUG] Parsed wishlist: {:?}", wishlist);
    WISHLISTS.lock().unwrap().push(wishlist.clone());
    Ok(Response::builder()
        .status(201)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&wishlist)?.into())?)
}

pub async fn handle_put(event: Request) -> Result<Response<Body>, Error> {
    let updated: Wishlist = serde_json::from_slice(event.body().as_ref())?;
    println!("[DEBUG] Updating wishlist with ID: {}", updated.id);
    let mut wishlists = WISHLISTS.lock().unwrap();
    println!("[DEBUG] Current wishlists before update: {:?}", wishlists);
    if let Some(pos) = wishlists.iter().position(|w| w.id == updated.id) {
        wishlists[pos] = updated.clone();
        println!("[DEBUG] Wishlist updated successfully");
        let updated_wishlist = wishlists[pos].clone();
        println!("[DEBUG] Current wishlists after update: {:?}", wishlists);
        Ok(Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&updated_wishlist)?.into())?)
    } else {
        println!("[DEBUG] Wishlist not found for update");
        Ok(Response::builder()
            .status(404)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&json!({"error": "Not Found"}))?.into())?)
    }
}

pub async fn handle_delete(event: Request) -> Result<Response<Body>, Error> {
    let body = event.body().as_ref();
    let id_map: std::collections::HashMap<String, String> = match serde_json::from_slice(body) {
        Ok(map) => map,
        Err(_) => {
            return Ok(Response::builder()
                .status(400)
                .body(Body::from("Invalid request body"))?)
        }
    };

    let id = match id_map.get("id") {
        Some(id) => id,
        None => {
            return Ok(Response::builder()
                .status(400)
                .body(Body::from("Missing id in request body"))?)
        }
    };

    println!("[DEBUG] Deleting wishlist with ID: {}", id);
    let mut wishlists = WISHLISTS.lock().unwrap();

    if let Some(pos) = wishlists.iter().position(|w| w.id == *id) {
        wishlists.remove(pos);
        println!("[DEBUG] Successfully deleted wishlist {}", id);
        Ok(Response::builder().status(200).body(Body::Empty)?)
    } else {
        println!("[DEBUG] Wishlist {} not found", id);
        Ok(Response::builder()
            .status(404)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&json!({"error": "Not Found"}))?.into())?)
    }
}
