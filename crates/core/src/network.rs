use crate::error::DownloadError;
use async_std::{
    fs::File,
    io::copy,
};
use isahc::config::RedirectPolicy;
use isahc::http::header::CONTENT_LENGTH;
use isahc::prelude::*;
use isahc::{HttpClient, Request, Response};
use once_cell::sync::Lazy;
use serde::Serialize;
use std::path::Path;

/// Shared `HttpClient`.
static HTTP_CLIENT: Lazy<HttpClient> = Lazy::new(|| {
    HttpClient::builder()
        .redirect_policy(RedirectPolicy::Follow)
        .max_connections_per_host(6)
        .build()
        .unwrap()
});

/// Grin Gui user-agent.
fn user_agent() -> String {
    format!("grin_gui/{}", env!("CARGO_PKG_VERSION"))
}

/// Generic request function.
pub async fn request_async<T: ToString>(
    url: T,
    headers: Vec<(&str, &str)>,
    timeout: Option<u64>,
) -> Result<Response<isahc::AsyncBody>, DownloadError> {
    // Sometimes a download url has a space.
    let url = url.to_string().replace(" ", "%20");

    let mut request = Request::builder().uri(url);

    for (name, value) in headers {
        request = request.header(name, value);
    }

    request = request.header("user-agent", &user_agent());

    if let Some(timeout) = timeout {
        request = request.timeout(std::time::Duration::from_secs(timeout));
    }

    Ok(HTTP_CLIENT.send_async(request.body(())?).await?)
}

// Generic function for posting Json data
pub(crate) async fn _post_json_async<T: ToString, D: Serialize>(
    url: T,
    data: D,
    headers: Vec<(&str, &str)>,
    timeout: Option<u64>,
) -> Result<Response<isahc::AsyncBody>, DownloadError> {
    let mut request = Request::builder()
        .method("POST")
        .uri(url.to_string())
        .header("content-type", "application/json");

    for (name, value) in headers {
        request = request.header(name, value);
    }

    request = request.header("user-agent", &user_agent());

    if let Some(timeout) = timeout {
        request = request.timeout(std::time::Duration::from_secs(timeout));
    }

    Ok(HTTP_CLIENT
        .send_async(request.body(serde_json::to_vec(&data)?)?)
        .await?)
}

/// Download a file from the internet
pub(crate) async fn download_file<T: ToString>(
    url: T,
    dest_file: &Path,
) -> Result<(), DownloadError> {
    let url = url.to_string();

    log::debug!("downloading file from {}", &url);

    let resp = request_async(&url, vec![("ACCEPT", "application/octet-stream")], None).await?;
    let (parts, mut body) = resp.into_parts();

    // If response length doesn't equal content length, full file wasn't downloaded
    // so error out
    {
        let content_length = parts
            .headers
            .get(CONTENT_LENGTH)
            .map(|v| v.to_str().unwrap_or_default())
            .unwrap_or_default()
            .parse::<u64>()
            .unwrap_or_default();

        let body_length = body.len().unwrap_or_default();

        if body_length != content_length {
            return Err(DownloadError::ContentLength {
                content_length,
                body_length,
            });
        }
    }

    let mut file = File::create(&dest_file).await?;

    copy(&mut body, &mut file).await?;

    log::debug!("file saved as {:?}", &dest_file);

    Ok(())
}
