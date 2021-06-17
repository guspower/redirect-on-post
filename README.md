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

### Verifying ureq client
1. Run standalone `warp` server located in `standalone/`
```
cd standalone
cargo run
```
2. Verify server is running
```
curl -v -F username=user001 http://localhost:3030/login
```
You should see output like: (look for "302 Found")
```
$ curl -v -F username=user001 http://localhost:3030/login
*   Trying ::1:3030...
* connect to ::1 port 3030 failed: Connection refused
*   Trying 127.0.0.1:3030...
* Connected to localhost (127.0.0.1) port 3030 (#0)
> POST /login HTTP/1.1
> Host: localhost:3030
> User-Agent: curl/7.77.0
> Accept: */*
> Content-Length: 150
> Content-Type: multipart/form-data; boundary=------------------------442a98f3de9a04e3
> 
* We are completely uploaded and fine
* Mark bundle as not supporting multiuse
< HTTP/1.1 302 Found
< location: /app
< content-length: 0
< date: Thu, 17 Jun 2021 12:33:13 GMT
< 
* Connection #0 to host localhost left intact
```

3. Modify `src/tests.rs` to call `warp` server instead of `wiremock`
```rust
    // let base_url = Url::parse(server.uri().as_str()).unwrap();
    let base_url = Url::parse("http://localhost:3030").unwrap();
```

4. Run tests
```redirect-on-post $ cargo test```:
You should get a clean run
   
```
$ cargo test
   Compiling redirect-on-post v0.1.0 (/home/gustavo/workspace/bugs/redirect-on-post)
    Finished test [unoptimized + debuginfo] target(s) in 4.65s
     Running unittests (target/debug/deps/redirect_on_post-b9808383b3d5488a)

running 1 test
test tests::redirect_on_form_submit ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
   
