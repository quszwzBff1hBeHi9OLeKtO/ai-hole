use crate::error::ProxyError;
use crate::processer::html;
use crate::role::RequestRole;
use crate::state::AppState;
use actix_web::{web, HttpRequest, HttpResponse};
use std::sync::atomic;

pub(crate) fn service(config: &mut web::ServiceConfig) {
    config.default_service(web::to(proxy_request));
}

async fn proxy_request(
    http_request: HttpRequest,
    http_client: web::Data<reqwest::Client>,
    body: web::Payload,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, ProxyError> {
    let Some(upstream) = http_request.headers().get("AI-Hole-Upstream") else {
        return Err(ProxyError::MissingUpstreamHeader);
    };
    let upstream = upstream
        .to_str()
        .map_err(|_error| ProxyError::NonAsciiHeaders)?;

    let role: RequestRole = http_request
        .headers()
        .get("AI-Hole-Role")
        .map(|s| s.to_str())
        .unwrap_or(Ok("bot"))
        .map_err(|_error| ProxyError::NonAsciiHeaders)?
        .try_into()
        .map_err(|_error| ProxyError::UnknownRole)?;

    let uri = http_request.uri().to_string();
    let uri = uri.strip_prefix("/proxy/").unwrap_or(&uri);

    let upstream_uri = format!("{upstream}/{uri}");
    let mut request_builder = http_client.request(http_request.method().clone(), upstream_uri);

    // Headers
    for (header, value) in http_request.headers() {
        request_builder = request_builder.header(header.to_string(), value.as_bytes());
    }

    let host = http_request
        .headers()
        .get("Host")
        .map(|host| host.to_str())
        .unwrap_or_else(|| Ok("unknown"))
        .map_err(|_error| ProxyError::NonAsciiHeaders)?;

    let peer_addr = http_request
        .peer_addr()
        .map(|addr| addr.to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let response = request_builder
        .body(body.to_bytes().await.expect("reading body failed mid-read"))
        .header(
            "Forwarded",
            format!("by=ai-hole;for={};host={};proto=http", peer_addr, host),
        )
        .send()
        .await
        .map_err(|_error| ProxyError::UpstreamUnreachable)?;

    let mut http_response = HttpResponse::Processing();
    http_response.status(response.status());

    for (header, value) in response.headers() {
        http_response.append_header((header, value));
    }

    // Processing
    let content_type = response
        .headers()
        .get("Content-Type")
        .map(|content_type| content_type.to_str())
        .unwrap_or_else(|| Ok("application/octet-stream"))
        .map_err(|_error| ProxyError::NonAsciiHeaders)?
        .to_string();

    let body = response.bytes().await.expect("failed to get body");

    tracing::debug!("Content type is {content_type} for {uri}");
    let body = match content_type
        .split(";")
        .nth(0)
        .expect("there has to be atleast one")
    {
        "text/html" => process_html(body, app_state, role),
        _ => body,
    };

    Ok(http_response.body(body))
}

fn process_html(body: web::Bytes, app_state: web::Data<AppState>, role: RequestRole) -> web::Bytes {
    match role {
        RequestRole::Bot => {
            app_state
                .stats
                .bot_requests
                .fetch_add(1, atomic::Ordering::Relaxed);
        }
        RequestRole::Human => {
            app_state
                .stats
                .human_requests
                .fetch_add(1, atomic::Ordering::Relaxed);
        }
    }
    let text = String::from_utf8_lossy(&body.to_vec()).to_string();
    let text = html::process_html(text, role, app_state); // TODO: Dynamic
    web::Bytes::copy_from_slice(text.expect("failed to rewrite html").as_bytes())
}
