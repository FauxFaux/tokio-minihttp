use futures::future;
use serde_json::json;
use tokio_minihttp::{Http, Request, Response};
use tokio_proto::TcpServer;
use tokio_service::Service;

struct Techempower;

impl Service for Techempower {
    type Request = Request;
    type Response = Response;
    type Error = std::io::Error;
    type Future = future::Ok<Response, std::io::Error>;

    fn call(&self, req: Request) -> Self::Future {
        let mut resp = Response::new();

        // Bare-bones router
        match req.path() {
            "/json" => {
                let json = serde_json::to_string(&json!({
                    "message": "Hello, World!"
                }))
                .unwrap();

                resp.header("Content-Type", "application/json").body(&json);
            }
            "/plaintext" => {
                resp.header("Content-Type", "text/plain")
                    .body("Hello, World!");
            }
            _ => {
                resp.status_code(404, "Not Found");
            }
        }

        future::ok(resp)
    }
}

fn main() {
    let addr = "0.0.0.0:8080".parse().unwrap();
    let mut srv = TcpServer::new(Http, addr);
    srv.threads(num_cpus::get());
    srv.serve(|| Ok(Techempower))
}
