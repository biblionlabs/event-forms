use crate::crypto::{
    decrypt_cookie_data, encrypt_cookie_data, fingerprint_type_from_fields, generate_fingerprint,
    generate_session_id, generate_user_id,
};
use crate::models::RequestInfo;
use serde_json::json;
use worker::Request;

const SESSION_COOKIE_NAME: &str = "ef_session";
const DATA_COOKIE_NAME: &str = "ef_data";
const FINGERPRINT_COOKIE_NAME: &str = "ef_fp";

/// Extract request info from Cloudflare headers
pub fn extract_request_info(req: &Request) -> RequestInfo {
    let headers = req.headers();

    let user_agent = headers.get("user-agent").ok().flatten();
    let device_type = detect_device_type(user_agent.as_deref());
    let (browser, os) = parse_user_agent(user_agent.as_deref());

    RequestInfo {
        ip_address: headers
            .get("cf-connecting-ip")
            .ok()
            .flatten()
            .or_else(|| headers.get("x-real-ip").ok().flatten()),
        user_agent,
        country: headers.get("cf-ipcountry").ok().flatten(),
        city: headers.get("cf-ipcity").ok().flatten(),
        region: headers.get("cf-region").ok().flatten(),
        timezone: headers.get("cf-timezone").ok().flatten(),
        device_type: Some(device_type),
        browser: Some(browser),
        os: Some(os),
        referrer: headers.get("referer").ok().flatten(),
    }
}

/// Detect device type from user agent
fn detect_device_type(user_agent: Option<&str>) -> String {
    let ua = user_agent.unwrap_or("").to_lowercase();

    if ua.contains("mobile") || ua.contains("android") && !ua.contains("tablet") {
        "mobile".to_string()
    } else if ua.contains("tablet") || ua.contains("ipad") {
        "tablet".to_string()
    } else {
        "desktop".to_string()
    }
}

/// Parse browser and OS from user agent
fn parse_user_agent(user_agent: Option<&str>) -> (String, String) {
    let ua = user_agent.unwrap_or("").to_lowercase();

    let browser = if ua.contains("firefox") {
        "Firefox"
    } else if ua.contains("edg") {
        "Edge"
    } else if ua.contains("chrome") {
        "Chrome"
    } else if ua.contains("safari") {
        "Safari"
    } else if ua.contains("opera") {
        "Opera"
    } else {
        "Unknown"
    }
    .to_string();

    let os = if ua.contains("windows") {
        "Windows"
    } else if ua.contains("mac os") || ua.contains("macos") {
        "macOS"
    } else if ua.contains("linux") {
        "Linux"
    } else if ua.contains("android") {
        "Android"
    } else if ua.contains("iphone") || ua.contains("ipad") {
        "iOS"
    } else {
        "Unknown"
    }
    .to_string();

    (browser, os)
}

/// Parse cookies from request
pub fn parse_cookies(req: &Request) -> std::collections::HashMap<String, String> {
    let mut cookies = std::collections::HashMap::new();

    if let Ok(Some(cookie_header)) = req.headers().get("cookie") {
        for cookie in cookie_header.split(';') {
            let parts: Vec<&str> = cookie.trim().splitn(2, '=').collect();
            if parts.len() == 2 {
                cookies.insert(parts[0].to_string(), parts[1].to_string());
            }
        }
    }

    cookies
}

/// Extract session data from cookies
pub fn extract_cookie_data(
    req: &Request,
    encryption_key: &str,
) -> (Option<String>, Option<String>, serde_json::Value) {
    let cookies = parse_cookies(req);

    // Get session ID
    let session_id = cookies.get(SESSION_COOKIE_NAME).cloned();

    // Get and decrypt fingerprint
    let fingerprint = cookies.get(FINGERPRINT_COOKIE_NAME).and_then(|encrypted| {
        decrypt_cookie_data(encrypted, encryption_key)
    });

    // Get and decrypt known fields data
    let known_fields = cookies
        .get(DATA_COOKIE_NAME)
        .and_then(|encrypted| decrypt_cookie_data(encrypted, encryption_key))
        .and_then(|json_str| serde_json::from_str(&json_str).ok())
        .unwrap_or_else(|| json!({}));

    (session_id, fingerprint, known_fields)
}

/// Build Set-Cookie header for session
pub fn build_session_cookie(session_id: &str, domain: &str) -> String {
    let domain_part = if domain.is_empty() {
        String::new()
    } else {
        format!("; Domain={}", domain)
    };

    format!(
        "{}={}; Path=/; HttpOnly; SameSite=Lax; Max-Age=31536000{}",
        SESSION_COOKIE_NAME, session_id, domain_part
    )
}

/// Build Set-Cookie header for fingerprint
pub fn build_fingerprint_cookie(fingerprint: &str, encryption_key: &str, domain: &str) -> String {
    let encrypted = encrypt_cookie_data(fingerprint, encryption_key);
    let domain_part = if domain.is_empty() {
        String::new()
    } else {
        format!("; Domain={}", domain)
    };

    format!(
        "{}={}; Path=/; HttpOnly; SameSite=Lax; Max-Age=31536000{}",
        FINGERPRINT_COOKIE_NAME, encrypted, domain_part
    )
}

/// Build Set-Cookie header for known fields data
pub fn build_data_cookie(data: &serde_json::Value, encryption_key: &str, domain: &str) -> String {
    let json_str = serde_json::to_string(data).unwrap_or_else(|_| "{}".to_string());
    let encrypted = encrypt_cookie_data(&json_str, encryption_key);
    let domain_part = if domain.is_empty() {
        String::new()
    } else {
        format!("; Domain={}", domain)
    };

    format!(
        "{}={}; Path=/; HttpOnly; SameSite=Lax; Max-Age=31536000{}",
        DATA_COOKIE_NAME, encrypted, domain_part
    )
}

/// Session manager for handling user sessions and fingerprinting
pub struct SessionManager {
    pub encryption_key: String,
    pub cookie_domain: String,
}

impl SessionManager {
    pub fn new(encryption_key: String, cookie_domain: String) -> Self {
        SessionManager {
            encryption_key,
            cookie_domain,
        }
    }

    /// Create a new session ID
    pub fn create_session_id(&self) -> String {
        generate_session_id()
    }

    /// Create a new user profile ID
    pub fn create_user_id(&self) -> String {
        generate_user_id()
    }

    /// Generate fingerprints for a set of field values based on configured fingerprint field combinations
    pub fn generate_fingerprints(
        &self,
        field_values: &serde_json::Value,
        fingerprint_configs: &[Vec<String>],
    ) -> Vec<(String, String)> {
        let mut fingerprints = Vec::new();

        for config in fingerprint_configs {
            // Check if we have all required fields for this fingerprint
            let mut has_all_fields = true;
            let mut fields: Vec<(&str, &str)> = Vec::new();

            for field_name in config {
                if let Some(value) = field_values.get(field_name).and_then(|v| v.as_str()) {
                    if !value.trim().is_empty() {
                        fields.push((field_name.as_str(), value));
                    } else {
                        has_all_fields = false;
                        break;
                    }
                } else {
                    has_all_fields = false;
                    break;
                }
            }

            if has_all_fields && !fields.is_empty() {
                let fp_type = fingerprint_type_from_fields(config);
                let fp_hash = generate_fingerprint(&fields, &self.encryption_key);
                fingerprints.push((fp_type, fp_hash));
            }
        }

        fingerprints
    }

    /// Check if a session is complete based on configured completion fields
    pub fn is_session_complete(
        &self,
        session_data: &serde_json::Value,
        completion_fields: &[String],
    ) -> bool {
        if completion_fields.is_empty() {
            return false;
        }

        for field in completion_fields {
            if let Some(value) = session_data.get(field).and_then(|v| v.as_str()) {
                if value.trim().is_empty() {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    /// Get fields that should be stored in cookies
    pub fn get_cookie_fields(
        &self,
        field_values: &serde_json::Value,
        cookie_field_names: &[String],
    ) -> serde_json::Value {
        let mut cookie_data = json!({});

        for field_name in cookie_field_names {
            if let Some(value) = field_values.get(field_name) {
                cookie_data[field_name] = value.clone();
            }
        }

        cookie_data
    }

    /// Merge known data from cookies with existing profile data
    pub fn merge_known_data(
        &self,
        existing: &serde_json::Value,
        new_data: &serde_json::Value,
    ) -> serde_json::Value {
        let mut merged = existing.clone();

        if let (Some(existing_obj), Some(new_obj)) = (merged.as_object_mut(), new_data.as_object())
        {
            for (key, value) in new_obj {
                // Only update if new value is not empty
                if let Some(str_val) = value.as_str() {
                    if !str_val.trim().is_empty() {
                        existing_obj.insert(key.clone(), value.clone());
                    }
                } else {
                    existing_obj.insert(key.clone(), value.clone());
                }
            }
        }

        merged
    }

    /// Build all necessary cookies for a response
    pub fn build_response_cookies(
        &self,
        session_id: &str,
        fingerprint: Option<&str>,
        cookie_data: Option<&serde_json::Value>,
    ) -> Vec<String> {
        let mut cookies = vec![build_session_cookie(session_id, &self.cookie_domain)];

        if let Some(fp) = fingerprint {
            cookies.push(build_fingerprint_cookie(
                fp,
                &self.encryption_key,
                &self.cookie_domain,
            ));
        }

        if let Some(data) = cookie_data {
            cookies.push(build_data_cookie(
                data,
                &self.encryption_key,
                &self.cookie_domain,
            ));
        }

        cookies
    }
}

/// Result of session resolution
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SessionResolution {
    pub session_id: String,
    pub is_new_session: bool,
    pub user_profile_id: Option<String>,
    pub is_new_user: bool,
    pub known_data: serde_json::Value,
    pub fingerprint: Option<String>,
}
