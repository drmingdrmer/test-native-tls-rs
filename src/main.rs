use openssl::ssl::{SslConnector, SslMethod};
use std::time::Instant;

fn main() {
    for (k, v) in std::env::vars() {
        println!("env: {}: {}", k, v);
    }

    let meth = SslMethod::tls();

    let t0 = Instant::now();
    let _x = SslConnector::builder(meth);
    let d = t0.elapsed();
    println!("=== SslConnector::builder costs: {:?}", d);
}
