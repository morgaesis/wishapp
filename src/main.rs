use lambda_http::{run, service_fn, Body, Error, Request, Response};
use wishlist_api::Wishlist;

async fn handle_request(event: Request) -> Result<Response<Body>, Error> {
    match event.method().as_str() {
        "GET" => handle_get(event).await,
        "POST" => handle_post(event).await,
        "PUT" => handle_put(event).await,
        "DELETE" => handle_delete(event).await,
        _ => Ok(Response::builder()
            .status(405)
            .body("Method Not Allowed".into())?),
    }
}

async fn handle_get(_event: Request) -> Result<Response<Body>, Error> {
    // TODO: Implement GET logic
    Ok(Response::builder().status(200).body("GET handler".into())?)
}

async fn handle_post(event: Request) -> Result<Response<Body>, Error> {
    // TODO: Implement POST logic
    Ok(Response::builder()
        .status(201)
        .body("POST handler".into())?)
}

async fn handle_put(_event: Request) -> Result<Response<Body>, Error> {
    // TODO: Implement PUT logic
    Ok(Response::builder().status(200).body("PUT handler".into())?)
}

async fn handle_delete(_event: Request) -> Result<Response<Body>, Error> {
    // TODO: Implement DELETE logic
    Ok(Response::builder().status(204).body("".into())?)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handle_request)).await
}
