#[cfg(test)]
mod tests;

use log::{error, info};
use std::time::Duration;
use url::Url;

fn main() {
    println!("Hello, world!");
}

fn login(url: &Url) -> Result<(), ureq::Error> {
    info!("Submitting login form ({})", url.as_str());

    let agent = ureq::builder().timeout(Duration::from_secs(5)).build();

    let response = agent.post(url.as_str()).send_form(
        &[
            ("username", "user001"),
            ("password", "pwd001")
        ],
    );

    match response {
        Ok(r) => {
            info!("Successful login {:?}", r.status());
            Ok(())
        }
        Err(error) => {
            error!("Failed login {:?}", error);
            Err(error)
        }
    }
}
