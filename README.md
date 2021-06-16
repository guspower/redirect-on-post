## Redirect On Post

### Usage
`cargo test`

### Error
```
running 1 test
test tests::redirect_on_form_submit ... FAILED

failures:

---- tests::redirect_on_form_submit stdout ----
thread '<unnamed>' panicked at 'Failed to read request body.: hyper::Error(Body, Custom { kind: UnexpectedEof, error: IncompleteBody })', /home/gustavo/.cargo/registry/src/github.com-1ecc6299db9ec823/wiremock-0.5.3/src/request.rs:103:14
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[2021-06-16T10:45:02Z ERROR redirect_on_post] Failed login Transport(Transport { kind: Io, message: None, url: Some(Url { scheme: "http", cannot_be_a_base: false, username: "", password: None, host: Some(Ipv4(127.0.0.1)), port: Some(36681), path: "/login", query: None, fragment: None }), source: Some(Custom { kind: TimedOut, error: Transport(Transport { kind: Io, message: Some("Error encountered in the status line"), url: None, source: Some(Custom { kind: TimedOut, error: "timed out reading response" }), response: None }) }), response: None })
thread 'tests::redirect_on_form_submit' panicked at 'called `Result::unwrap()` on an `Err` value: Transport(Transport { kind: Io, message: None, url: Some(Url { scheme: "http", cannot_be_a_base: false, username: "", password: None, host: Some(Ipv4(127.0.0.1)), port: Some(36681), path: "/login", query: None, fragment: None }), source: Some(Custom { kind: TimedOut, error: Transport(Transport { kind: Io, message: Some("Error encountered in the status line"), url: None, source: Some(Custom { kind: TimedOut, error: "timed out reading response" }), response: None }) }), response: None })', src/tests.rs:37:23
```