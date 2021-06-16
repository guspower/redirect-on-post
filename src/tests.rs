use super::*;

use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{header, header_exists, method, path};

#[async_std::test]
async fn redirect_on_form_submit() {
    let _ = env_logger::builder().is_test(true).try_init();
    let server = MockServer::start().await;

    info!("WireMock server running on {}", server.uri());

    let base_url = Url::parse(server.uri().as_str()).unwrap();
    let login_url = base_url.join("/login").unwrap();

    Mock::given(method("POST"))
        .and(path("/login"))
        .and(header("Content-Type", "application/x-www-form-urlencoded"))
        .and(header_exists("Content-Length"))
        .respond_with(ResponseTemplate::new(302)
            .set_body_string("<empty>")
            .append_header("Location", "/app")
        )
        .up_to_n_times(1)
        .named("Login POST w/ redirect")
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/app"))
        .respond_with(ResponseTemplate::new(204))
        .up_to_n_times(1)
        .named("Login GET after redirect")
        .mount(&server)
        .await;

    login(&login_url).unwrap();
}
