use lambda_http::{Body, Error, Request, Response};

pub async fn handle_get(_event: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder().status(200).body("GET handler".into())?)
}

pub async fn handle_post(_event: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(201)
        .body("POST handler".into())?)
}

pub async fn handle_put(_event: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder().status(200).body("PUT handler".into())?)
}

pub async fn handle_delete(_event: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder().status(204).body("".into())?)
}
