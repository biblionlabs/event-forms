use serde::{Deserialize, Serialize};

// ============================================================================
// Form Models
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Form {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub slug: String,
    pub is_active: bool,
    pub tags: Vec<String>,
    pub session_complete_fields: Vec<String>,
    pub cookie_fields: Vec<String>,
    pub fingerprint_fields: Vec<Vec<String>>,
    pub thank_you_title: String,
    pub thank_you_message: String,
    pub thank_you_image_url: Option<String>,
    pub primary_color: String,
    pub logo_url: Option<String>,
    pub background_color: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormStep {
    pub id: String,
    pub form_id: String,
    pub step_number: i32,
    pub title: String,
    pub description: Option<String>,
    pub completion_title: Option<String>,
    pub completion_message: Option<String>,
    pub completion_image_url: Option<String>,
    pub show_completion_message: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormField {
    pub id: String,
    pub step_id: String,
    pub form_id: String,
    pub field_name: String,
    pub field_type: FieldType,
    pub label: String,
    pub placeholder: Option<String>,
    pub help_text: Option<String>,
    pub validation: FieldValidation,
    pub options: Option<Vec<FieldOption>>,
    pub is_required: bool,
    pub is_identifier: bool,
    pub store_in_cookie: bool,
    pub display_order: i32,
    pub conditional: Option<FieldConditional>,
    pub default_value: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldType {
    Text,
    Email,
    Phone,
    Number,
    Select,
    Radio,
    Checkbox,
    Textarea,
    Date,
    Hidden,
}

impl FieldType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "email" => FieldType::Email,
            "phone" => FieldType::Phone,
            "number" => FieldType::Number,
            "select" => FieldType::Select,
            "radio" => FieldType::Radio,
            "checkbox" => FieldType::Checkbox,
            "textarea" => FieldType::Textarea,
            "date" => FieldType::Date,
            "hidden" => FieldType::Hidden,
            _ => FieldType::Text,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            FieldType::Text => "text",
            FieldType::Email => "email",
            FieldType::Phone => "phone",
            FieldType::Number => "number",
            FieldType::Select => "select",
            FieldType::Radio => "radio",
            FieldType::Checkbox => "checkbox",
            FieldType::Textarea => "textarea",
            FieldType::Date => "date",
            FieldType::Hidden => "hidden",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FieldValidation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldOption {
    pub value: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldConditional {
    pub field_name: String,
    pub operator: String, // eq, neq, contains, gt, lt
    pub value: String,
}

// ============================================================================
// User Models
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub fingerprints: serde_json::Value,
    pub known_data: serde_json::Value,
    pub first_seen_at: String,
    pub first_form_id: Option<String>,
    pub first_device_info: Option<String>,
    pub first_geo_info: Option<String>,
    pub last_seen_at: String,
    pub last_form_id: Option<String>,
    pub last_device_info: Option<String>,
    pub last_geo_info: Option<String>,
    pub total_forms_completed: i32,
    pub total_interactions: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub id: String,
    pub user_profile_id: Option<String>,
    pub is_complete: bool,
    pub completed_fields: Vec<String>,
    pub session_data: serde_json::Value,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub timezone: Option<String>,
    pub device_type: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub started_at: String,
    pub last_activity_at: String,
    pub expires_at: Option<String>,
    pub created_at: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFingerprint {
    pub id: String,
    pub user_profile_id: String,
    pub fingerprint_type: String,
    pub fingerprint_hash: String,
    pub source_values: Option<String>,
    pub created_at: String,
}

// ============================================================================
// Response Models
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormResponse {
    pub id: String,
    pub form_id: String,
    pub user_session_id: Option<String>,
    pub user_profile_id: Option<String>,
    pub status: ResponseStatus,
    pub current_step: i32,
    pub total_steps: Option<i32>,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub ip_address: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub device_type: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseStatus {
    InProgress,
    Completed,
    Abandoned,
}

impl ResponseStatus {
    pub fn from_str(s: &str) -> Self {
        match s {
            "completed" => ResponseStatus::Completed,
            "abandoned" => ResponseStatus::Abandoned,
            _ => ResponseStatus::InProgress,
        }
    }

    #[allow(dead_code)]
    pub fn as_str(&self) -> &str {
        match self {
            ResponseStatus::InProgress => "in_progress",
            ResponseStatus::Completed => "completed",
            ResponseStatus::Abandoned => "abandoned",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldResponse {
    pub id: String,
    pub form_response_id: String,
    pub form_field_id: String,
    pub step_id: String,
    pub field_name: String,
    pub field_value: Option<String>,
    pub submitted_at: String,
}

// ============================================================================
// Analytics Models
// ============================================================================

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanEvent {
    pub id: String,
    pub form_id: String,
    pub step_number: i32,
    pub user_session_id: Option<String>,
    pub user_profile_id: Option<String>,
    pub is_new_user: bool,
    pub is_new_session: bool,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub timezone: Option<String>,
    pub device_type: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub referrer: Option<String>,
    pub scanned_at: String,
}

// ============================================================================
// Request/Response DTOs
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFormRequest {
    pub name: String,
    pub description: Option<String>,
    pub slug: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub session_complete_fields: Vec<String>,
    #[serde(default)]
    pub cookie_fields: Vec<String>,
    #[serde(default)]
    pub fingerprint_fields: Vec<Vec<String>>,
    #[serde(default = "default_thank_you_title")]
    pub thank_you_title: String,
    #[serde(default = "default_thank_you_message")]
    pub thank_you_message: String,
    pub thank_you_image_url: Option<String>,
    #[serde(default = "default_primary_color")]
    pub primary_color: String,
    pub logo_url: Option<String>,
    #[serde(default = "default_background_color")]
    pub background_color: String,
}

fn default_thank_you_title() -> String {
    "Thank you!".to_string()
}

fn default_thank_you_message() -> String {
    "Your response has been recorded.".to_string()
}

fn default_primary_color() -> String {
    "#3B82F6".to_string()
}

fn default_background_color() -> String {
    "#F9FAFB".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFormRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub slug: Option<String>,
    pub is_active: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub session_complete_fields: Option<Vec<String>>,
    pub cookie_fields: Option<Vec<String>>,
    pub fingerprint_fields: Option<Vec<Vec<String>>>,
    pub thank_you_title: Option<String>,
    pub thank_you_message: Option<String>,
    pub thank_you_image_url: Option<String>,
    pub primary_color: Option<String>,
    pub logo_url: Option<String>,
    pub background_color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStepRequest {
    pub title: String,
    pub description: Option<String>,
    pub completion_title: Option<String>,
    pub completion_message: Option<String>,
    pub completion_image_url: Option<String>,
    #[serde(default)]
    pub show_completion_message: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateStepRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub step_number: Option<i32>,
    pub completion_title: Option<String>,
    pub completion_message: Option<String>,
    pub completion_image_url: Option<String>,
    pub show_completion_message: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFieldRequest {
    pub field_name: String,
    pub field_type: String,
    pub label: String,
    pub placeholder: Option<String>,
    pub help_text: Option<String>,
    #[serde(default)]
    pub validation: FieldValidation,
    pub options: Option<Vec<FieldOption>>,
    #[serde(default)]
    pub is_required: bool,
    #[serde(default)]
    pub is_identifier: bool,
    #[serde(default)]
    pub store_in_cookie: bool,
    #[serde(default)]
    pub display_order: i32,
    pub conditional: Option<FieldConditional>,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFieldRequest {
    pub field_name: Option<String>,
    pub field_type: Option<String>,
    pub label: Option<String>,
    pub placeholder: Option<String>,
    pub help_text: Option<String>,
    pub validation: Option<FieldValidation>,
    pub options: Option<Vec<FieldOption>>,
    pub is_required: Option<bool>,
    pub is_identifier: Option<bool>,
    pub store_in_cookie: Option<bool>,
    pub display_order: Option<i32>,
    pub conditional: Option<FieldConditional>,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitStepRequest {
    pub fields: serde_json::Value,
}

// ============================================================================
// Dashboard Analytics DTOs
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormStats {
    pub form_id: String,
    pub form_name: String,
    pub total_scans: i64,
    pub unique_users: i64,
    pub new_users: i64,
    pub returning_users: i64,
    pub completed_responses: i64,
    pub in_progress_responses: i64,
    pub abandoned_responses: i64,
    pub completion_rate: f64,
    pub step_funnel: Vec<StepFunnelItem>,
    pub device_breakdown: DeviceBreakdown,
    pub geo_breakdown: Vec<GeoItem>,
    pub hourly_scans: Vec<HourlyItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepFunnelItem {
    pub step_number: i32,
    pub step_title: String,
    pub started: i64,
    pub completed: i64,
    pub drop_off_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceBreakdown {
    pub mobile: i64,
    pub tablet: i64,
    pub desktop: i64,
    pub unknown: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoItem {
    pub country: String,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourlyItem {
    pub hour: String,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInteractionHistory {
    pub user_profile_id: String,
    pub known_data: serde_json::Value,
    pub total_forms: i32,
    pub total_interactions: i32,
    pub first_seen: String,
    pub last_seen: String,
    pub interactions: Vec<UserInteraction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInteraction {
    pub interaction_type: String, // scan, submit, complete
    pub form_id: String,
    pub form_name: String,
    pub step_number: Option<i32>,
    pub timestamp: String,
    pub device_type: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
}

// ============================================================================
// Device/Geo Info
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RequestInfo {
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub timezone: Option<String>,
    pub device_type: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub referrer: Option<String>,
}

impl RequestInfo {
    pub fn to_device_json(&self) -> String {
        serde_json::json!({
            "device_type": self.device_type,
            "browser": self.browser,
            "os": self.os,
            "user_agent": self.user_agent
        })
        .to_string()
    }

    pub fn to_geo_json(&self) -> String {
        serde_json::json!({
            "country": self.country,
            "city": self.city,
            "region": self.region,
            "timezone": self.timezone,
            "ip_address": self.ip_address
        })
        .to_string()
    }
}

// ============================================================================
// Cookie Data
// ============================================================================

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CookieData {
    pub session_id: Option<String>,
    pub fingerprint: Option<String>,
    pub known_fields: serde_json::Value,
}

// ============================================================================
// API Response wrapper
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: &str) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            error: Some(message.to_string()),
        }
    }
}
