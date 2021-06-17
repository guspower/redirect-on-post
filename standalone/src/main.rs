use std::error::Error;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let login_with_redirect = warp::path!("login")
        .and(warp::post())
        .map(|| {
            warp::reply::with_header(
                warp::http::StatusCode::FOUND,
                warp::http::header::LOCATION,
                warp::http::HeaderValue::from_static("/app"),
            )
        });

    let app = warp::path!("app")
        .and(warp::get())
        .map(|| {
            warp::reply::json(&String::from("application-data"))
        });

    let routes = login_with_redirect.or(app);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}

