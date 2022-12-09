This is a demo showing how `cargo` pollutes environment variables and break a test case.


On linux, `cargo` leaks env variables such as `SSL_CERT_FILE` and `SSL_CERT_DIR`.
[openssl](https://github.com/sfackler/rust-openssl) will loads certificates from
`SSL_CERT_FILE` is this env var is set.

Thus if a program(e.g., unittest) depends on `openssl`, `cargo build &&
./target/debug/bin-name` may work very fine, but `cargo run` may time out,
because loading certificates takes several dozen milliseconds(`~ 30ms to 70ms`).


`cargo run` in this repo shows the env vars a program sees and the time it takes
calling `openssl::ssl::SslConnector::builder()`.

**On linux**:

```text
cargo run

SSL_CERT_DIR: /usr/lib/ssl/certs
SSL_CERT_FILE: /usr/lib/ssl/certs/ca-certificates.crt
...
=== : 66.382982ms
```


**On my m1 mac**:

```text
cargo run

...
=== : 9.23432ms
```
