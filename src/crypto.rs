use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Generate a fingerprint hash from a set of field values
/// The fingerprint is a HMAC-SHA256 hash of the normalized, sorted field values
pub fn generate_fingerprint(fields: &[(&str, &str)], secret_key: &str) -> String {
    // Sort fields by name and normalize values (lowercase, trim)
    let mut sorted_fields: Vec<_> = fields
        .iter()
        .map(|(k, v)| (k.to_lowercase(), v.to_lowercase().trim().to_string()))
        .collect();
    sorted_fields.sort_by(|a, b| a.0.cmp(&b.0));

    // Create a canonical string representation
    let canonical: String = sorted_fields
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("&");

    // Generate HMAC-SHA256
    let mut mac =
        HmacSha256::new_from_slice(secret_key.as_bytes()).expect("HMAC can take key of any size");
    mac.update(canonical.as_bytes());
    let result = mac.finalize();

    // Return base64-encoded hash
    URL_SAFE_NO_PAD.encode(result.into_bytes())
}

/// Generate a fingerprint type string from field names
/// e.g., ["email"] -> "email", ["name", "school", "age"] -> "age_name_school"
pub fn fingerprint_type_from_fields(fields: &[String]) -> String {
    let mut sorted: Vec<_> = fields.iter().map(|s| s.to_lowercase()).collect();
    sorted.sort();
    sorted.join("_")
}

/// Encrypt data using HMAC-SHA256 for integrity and base64 encoding
/// Format: base64(data) + "." + base64(hmac(data))
pub fn encrypt_cookie_data(data: &str, secret_key: &str) -> String {
    let encoded_data = URL_SAFE_NO_PAD.encode(data.as_bytes());

    let mut mac =
        HmacSha256::new_from_slice(secret_key.as_bytes()).expect("HMAC can take key of any size");
    mac.update(data.as_bytes());
    let signature = URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes());

    format!("{}.{}", encoded_data, signature)
}

/// Decrypt and verify cookie data
/// Returns None if verification fails
pub fn decrypt_cookie_data(encrypted: &str, secret_key: &str) -> Option<String> {
    let parts: Vec<&str> = encrypted.split('.').collect();
    if parts.len() != 2 {
        return None;
    }

    let encoded_data = parts[0];
    let signature = parts[1];

    // Decode the data
    let data_bytes = URL_SAFE_NO_PAD.decode(encoded_data).ok()?;
    let data = String::from_utf8(data_bytes).ok()?;

    // Verify the signature
    let mut mac =
        HmacSha256::new_from_slice(secret_key.as_bytes()).expect("HMAC can take key of any size");
    mac.update(data.as_bytes());
    let expected_signature = URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes());

    if signature == expected_signature {
        Some(data)
    } else {
        None
    }
}

/// Generate a secure session ID
pub fn generate_session_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Generate a secure user profile ID
pub fn generate_user_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Hash a single value (for simple fingerprints like email)
#[allow(dead_code)]
pub fn hash_value(value: &str, secret_key: &str) -> String {
    let normalized = value.to_lowercase().trim().to_string();

    let mut mac =
        HmacSha256::new_from_slice(secret_key.as_bytes()).expect("HMAC can take key of any size");
    mac.update(normalized.as_bytes());
    let result = mac.finalize();

    URL_SAFE_NO_PAD.encode(result.into_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fingerprint_generation() {
        let fields = vec![("email", "test@example.com"), ("name", "John Doe")];
        let fp1 = generate_fingerprint(&fields, "secret");

        // Same fields in different order should produce same fingerprint
        let fields2 = vec![("name", "John Doe"), ("email", "test@example.com")];
        let fp2 = generate_fingerprint(&fields2, "secret");

        assert_eq!(fp1, fp2);

        // Different values should produce different fingerprint
        let fields3 = vec![("email", "other@example.com"), ("name", "John Doe")];
        let fp3 = generate_fingerprint(&fields3, "secret");

        assert_ne!(fp1, fp3);
    }

    #[test]
    fn test_cookie_encryption() {
        let data = r#"{"session_id":"123","email":"test@example.com"}"#;
        let secret = "my-secret-key";

        let encrypted = encrypt_cookie_data(data, secret);
        let decrypted = decrypt_cookie_data(&encrypted, secret);

        assert_eq!(decrypted, Some(data.to_string()));

        // Tampered data should fail verification
        let tampered = format!("tampered.{}", encrypted.split('.').nth(1).unwrap());
        assert_eq!(decrypt_cookie_data(&tampered, secret), None);
    }

    #[test]
    fn test_fingerprint_type() {
        let fields = vec!["name".to_string(), "school".to_string(), "age".to_string()];
        let fp_type = fingerprint_type_from_fields(&fields);
        assert_eq!(fp_type, "age_name_school");
    }
}
