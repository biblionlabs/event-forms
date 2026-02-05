use base64::{engine::general_purpose::STANDARD, Engine};
use worker::{Request, Response, Result};

/// Check if request has valid basic authentication
pub fn check_basic_auth(req: &Request, username: &str, password: &str) -> bool {
    let auth_header = match req.headers().get("authorization") {
        Ok(Some(h)) => h,
        _ => return false,
    };

    if !auth_header.starts_with("Basic ") {
        return false;
    }

    let encoded = &auth_header[6..];
    let decoded = match STANDARD.decode(encoded) {
        Ok(d) => d,
        Err(_) => return false,
    };

    let credentials = match String::from_utf8(decoded) {
        Ok(c) => c,
        Err(_) => return false,
    };

    let parts: Vec<&str> = credentials.splitn(2, ':').collect();
    if parts.len() != 2 {
        return false;
    }

    parts[0] == username && parts[1] == password
}

/// Return 401 Unauthorized response with WWW-Authenticate header
pub fn unauthorized_response() -> Result<Response> {
    let headers = worker::Headers::new();
    headers.set("WWW-Authenticate", "Basic realm=\"Event Forms Admin\"")?;
    headers.set("Content-Type", "text/html; charset=utf-8")?;

    let body = r#"<!DOCTYPE html>
<html>
<head>
    <title>401 Unauthorized</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; display: flex; justify-content: center; align-items: center; height: 100vh; margin: 0; background: #f3f4f6; }
        .container { text-align: center; padding: 2rem; background: white; border-radius: 8px; box-shadow: 0 1px 3px rgba(0,0,0,0.1); }
        h1 { color: #ef4444; margin-bottom: 0.5rem; }
        p { color: #6b7280; }
    </style>
</head>
<body>
    <div class="container">
        <h1>401 Unauthorized</h1>
        <p>Please provide valid credentials to access this resource.</p>
    </div>
</body>
</html>"#;

    Response::from_html(body).map(|mut r| {
        *r.headers_mut() = headers;
        r.with_status(401)
    })
}

/// Middleware to protect admin routes
pub async fn require_auth(req: &Request, username: &str, password: &str) -> Option<Response> {
    if !check_basic_auth(req, username, password) {
        Some(unauthorized_response().unwrap_or_else(|_| Response::error("Unauthorized", 401).unwrap()))
    } else {
        None
    }
}
