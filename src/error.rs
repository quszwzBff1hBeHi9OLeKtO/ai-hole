use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use std::fmt::Display;

#[derive(Debug)]
pub(crate) enum ProxyError {
    MissingUpstreamHeader,
    NonAsciiHeaders,
    UpstreamUnreachable,
    UnknownAction,
    UnknownRole,
}

impl ProxyError {
    fn error_text(&self) -> String {
        self.to_string()
    }
}

impl Display for ProxyError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::MissingUpstreamHeader => "missing upstream header",
            Self::NonAsciiHeaders => "not ascii headers",
            Self::UpstreamUnreachable => "upstream is unreachable",
            Self::UnknownAction => "unknown action",
            Self::UnknownRole => "unknown role",
        };
        formatter.write_str(text)
    }
}

impl ResponseError for ProxyError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match &self {
            Self::MissingUpstreamHeader => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NonAsciiHeaders => StatusCode::BAD_REQUEST,
            Self::UpstreamUnreachable => StatusCode::SERVICE_UNAVAILABLE,
            Self::UnknownAction => StatusCode::INTERNAL_SERVER_ERROR,
            Self::UnknownRole => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::Processing()
            .status(self.status_code())
            .insert_header(("AI-Hole-Error", "true"))
            .body(self.error_text())
            .map_into_boxed_body()
    }
}
