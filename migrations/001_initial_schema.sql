-- Migration 001: Initial schema
-- Event Forms D1 Schema
-- Multi-step forms with intelligent user matching

-- Forms table - main form configuration
CREATE TABLE IF NOT EXISTS forms (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    slug TEXT UNIQUE NOT NULL,
    is_active INTEGER DEFAULT 1,
    -- Tags for segmentation (JSON array)
    tags TEXT DEFAULT '[]',
    -- Thank you configuration
    thank_you_title TEXT DEFAULT '¡Gracias!',
    thank_you_message TEXT DEFAULT 'Tu respuesta ha sido registrada.',
    thank_you_image_url TEXT,
    -- Styling
    primary_color TEXT DEFAULT '#3B82F6',
    logo_url TEXT,
    background_color TEXT DEFAULT '#F9FAFB',
    -- Metadata
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);

-- Form steps - each step in a multi-step form
CREATE TABLE IF NOT EXISTS form_steps (
    id TEXT PRIMARY KEY,
    form_id TEXT NOT NULL,
    step_number INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    -- Step completion message (shown after completing this step)
    completion_title TEXT,
    completion_message TEXT,
    completion_image_url TEXT,
    show_completion_message INTEGER DEFAULT 0,
    -- Metadata
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (form_id) REFERENCES forms(id) ON DELETE CASCADE,
    UNIQUE(form_id, step_number)
);

-- Form fields - individual fields within a step
CREATE TABLE IF NOT EXISTS form_fields (
    id TEXT PRIMARY KEY,
    step_id TEXT NOT NULL,
    form_id TEXT NOT NULL,
    field_name TEXT NOT NULL,
    field_type TEXT NOT NULL, -- text, email, phone, number, textarea, select, radio, checkbox, date, hidden
    label TEXT NOT NULL,
    placeholder TEXT,
    help_text TEXT,
    -- Validation rules (JSON object with min_length, max_length, pattern, etc.)
    validation TEXT DEFAULT '{}',
    -- Options for select/radio/checkbox (JSON array of {value, label} objects)
    options TEXT,
    is_required INTEGER DEFAULT 0,
    -- For user identification/fingerprinting
    is_identifier INTEGER DEFAULT 0,
    -- Store this field value in cookies for pre-filling
    store_in_cookie INTEGER DEFAULT 0,
    display_order INTEGER DEFAULT 0,
    -- Conditional display rules (JSON)
    conditional TEXT,
    default_value TEXT,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (step_id) REFERENCES form_steps(id) ON DELETE CASCADE,
    FOREIGN KEY (form_id) REFERENCES forms(id) ON DELETE CASCADE
);

-- User profiles - unified user data across all forms
CREATE TABLE IF NOT EXISTS user_profiles (
    id TEXT PRIMARY KEY,
    -- Known data collected from identifier fields (JSON)
    known_data TEXT DEFAULT '{}',
    first_seen_at TEXT DEFAULT CURRENT_TIMESTAMP,
    first_form_id TEXT,
    first_device_info TEXT,
    first_geo_info TEXT,
    last_seen_at TEXT DEFAULT CURRENT_TIMESTAMP,
    last_form_id TEXT,
    last_device_info TEXT,
    last_geo_info TEXT,
    total_forms_completed INTEGER DEFAULT 0,
    total_interactions INTEGER DEFAULT 0,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);

-- User sessions - individual browsing sessions
CREATE TABLE IF NOT EXISTS user_sessions (
    id TEXT PRIMARY KEY,
    user_profile_id TEXT,
    is_complete INTEGER DEFAULT 0,
    -- Data collected during this session (JSON)
    session_data TEXT DEFAULT '{}',
    ip_address TEXT,
    user_agent TEXT,
    country TEXT,
    city TEXT,
    region TEXT,
    timezone TEXT,
    device_type TEXT,
    browser TEXT,
    os TEXT,
    started_at TEXT DEFAULT CURRENT_TIMESTAMP,
    last_activity_at TEXT DEFAULT CURRENT_TIMESTAMP,
    expires_at TEXT,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_profile_id) REFERENCES user_profiles(id) ON DELETE SET NULL
);

-- Form responses - tracks form submission progress
CREATE TABLE IF NOT EXISTS form_responses (
    id TEXT PRIMARY KEY,
    form_id TEXT NOT NULL,
    user_session_id TEXT,
    user_profile_id TEXT,
    status TEXT DEFAULT 'in_progress', -- in_progress, completed, abandoned
    current_step INTEGER DEFAULT 1,
    total_steps INTEGER,
    started_at TEXT DEFAULT CURRENT_TIMESTAMP,
    completed_at TEXT,
    ip_address TEXT,
    country TEXT,
    city TEXT,
    device_type TEXT,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (form_id) REFERENCES forms(id) ON DELETE CASCADE,
    FOREIGN KEY (user_session_id) REFERENCES user_sessions(id) ON DELETE SET NULL,
    FOREIGN KEY (user_profile_id) REFERENCES user_profiles(id) ON DELETE SET NULL
);

-- Field responses - individual field values
CREATE TABLE IF NOT EXISTS field_responses (
    id TEXT PRIMARY KEY,
    form_response_id TEXT NOT NULL,
    form_field_id TEXT NOT NULL,
    step_id TEXT NOT NULL,
    field_name TEXT NOT NULL,
    field_value TEXT,
    submitted_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (form_response_id) REFERENCES form_responses(id) ON DELETE CASCADE,
    FOREIGN KEY (form_field_id) REFERENCES form_fields(id) ON DELETE CASCADE,
    FOREIGN KEY (step_id) REFERENCES form_steps(id) ON DELETE CASCADE
);

-- QR scan events - analytics for each scan
CREATE TABLE IF NOT EXISTS scan_events (
    id TEXT PRIMARY KEY,
    form_id TEXT NOT NULL,
    step_number INTEGER NOT NULL,
    user_session_id TEXT,
    user_profile_id TEXT,
    is_new_user INTEGER DEFAULT 1,
    is_new_session INTEGER DEFAULT 1,
    ip_address TEXT,
    user_agent TEXT,
    country TEXT,
    city TEXT,
    region TEXT,
    timezone TEXT,
    device_type TEXT,
    browser TEXT,
    os TEXT,
    referrer TEXT,
    scanned_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (form_id) REFERENCES forms(id) ON DELETE CASCADE,
    FOREIGN KEY (user_session_id) REFERENCES user_sessions(id) ON DELETE SET NULL,
    FOREIGN KEY (user_profile_id) REFERENCES user_profiles(id) ON DELETE SET NULL
);

-- User fingerprints - for matching returning users
CREATE TABLE IF NOT EXISTS user_fingerprints (
    id TEXT PRIMARY KEY,
    user_profile_id TEXT NOT NULL,
    fingerprint_type TEXT NOT NULL, -- email, phone, name_school, etc.
    fingerprint_hash TEXT NOT NULL,
    source_values TEXT,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_profile_id) REFERENCES user_profiles(id) ON DELETE CASCADE
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_forms_slug ON forms(slug);
CREATE INDEX IF NOT EXISTS idx_forms_active ON forms(is_active);
CREATE INDEX IF NOT EXISTS idx_form_steps_form ON form_steps(form_id, step_number);
CREATE INDEX IF NOT EXISTS idx_form_fields_step ON form_fields(step_id, display_order);
CREATE INDEX IF NOT EXISTS idx_form_fields_form ON form_fields(form_id);
CREATE INDEX IF NOT EXISTS idx_form_fields_identifier ON form_fields(is_identifier);
CREATE INDEX IF NOT EXISTS idx_form_fields_cookie ON form_fields(store_in_cookie);
CREATE INDEX IF NOT EXISTS idx_user_sessions_profile ON user_sessions(user_profile_id);
CREATE INDEX IF NOT EXISTS idx_form_responses_form ON form_responses(form_id);
CREATE INDEX IF NOT EXISTS idx_form_responses_session ON form_responses(user_session_id);
CREATE INDEX IF NOT EXISTS idx_form_responses_status ON form_responses(status);
CREATE INDEX IF NOT EXISTS idx_field_responses_response ON field_responses(form_response_id);
CREATE INDEX IF NOT EXISTS idx_scan_events_form ON scan_events(form_id);
CREATE INDEX IF NOT EXISTS idx_scan_events_session ON scan_events(user_session_id);
CREATE INDEX IF NOT EXISTS idx_scan_events_date ON scan_events(scanned_at);
CREATE INDEX IF NOT EXISTS idx_user_fingerprints_hash ON user_fingerprints(fingerprint_hash);
CREATE INDEX IF NOT EXISTS idx_user_fingerprints_profile ON user_fingerprints(user_profile_id);
