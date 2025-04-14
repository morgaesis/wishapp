use lambda_http::{Body, Error, Request, Response};

pub async fn handle_get(event: Request) -> Result<Response<Body>, Error> {
    match event.uri().path() {
        "/health" => Ok(Response::builder().status(200).body("OK".into())?),
        _ => Ok(Response::builder().status(404).body("Not Found".into())?),
    }
}

pub async fn handle_post(_event: Request) -> Result<Response<Body>, Error> {
    unimplemented!()
}

pub async fn handle_put(_event: Request) -> Result<Response<Body>, Error> {
    unimplemented!()
}

pub async fn handle_delete(_event: Request) -> Result<Response<Body>, Error> {
    unimplemented!()
}
