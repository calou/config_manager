use std::collections::{BTreeSet, HashMap};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use hyper::{Body, Request, Response, Server};
use hyper::service::{Service};
use url::form_urlencoded;

use crate::reserved_ports::ReservedPorts;

mod reserved_ports;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr).serve(MakeSvc { reserved_ports: ReservedPorts { entries: BTreeSet::new() } });
    println!("Listening on http://{}", addr);

    server.await?;
    Ok(())
}

struct Svc {
    reserved_ports: ReservedPorts,
}

impl Service<Request<Body>> for Svc {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        fn mk_response(s: String) -> Result<Response<Body>, hyper::Error> {
            Ok(Response::builder().body(Body::from(s)).unwrap())
        }

        let res = match req.uri().path() {
            "/next" => {
                let from = Self::get_from_parameter_value(req);
                mk_response(format!("{:?}", self.reserved_ports.next(from)))
            },
            "/next/reserve" => {
                let from = Self::get_from_parameter_value(req);
                mk_response(format!("{:?}", self.reserved_ports.reserve_next(from)))
            },
            _ => return Box::pin(async { mk_response("oh no! not found".into()) }),
        };

        Box::pin(async { res })
    }
}

impl Svc {
    fn get_from_parameter_value(req: Request<Body>) -> Option<u32> {
        let from = if let Some(query) = req.uri().query() {
            let params = form_urlencoded::parse(query.as_bytes())
                .into_owned()
                .collect::<HashMap<String, String>>();
            let option = params.get("from");
            let value: u32 =  option.unwrap().parse().unwrap();
            Some(value)
        } else {
            None
        };
        from
    }
}

struct MakeSvc {
    reserved_ports: ReservedPorts,
}

impl<T> Service<T> for MakeSvc {
    type Response = Svc;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: T) -> Self::Future {
        let store = self.reserved_ports.clone();
        let fut = async move { Ok(Svc { reserved_ports: reserved }) };
        Box::pin(fut)
    }
}