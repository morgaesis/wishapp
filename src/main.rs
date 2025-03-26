mod backend;
mod frontend;

fn main() {
    backend::start_backend();
    frontend::start_frontend();
}
