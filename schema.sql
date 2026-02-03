-- Event Forms D1 Schema
-- Multi-step forms with intelligent user matching

-- Forms table - main form configuration
CREATE TABLE IF NOT EXISTS forms (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    slug TEXT UNIQUE NOT NULL,
    is_active INTEGER DEFAULT 1,
    -- Session completion configuration (JSON array of field names that define a complete session)
    session_complete_fields TEXT DEFAULT '["email"]',
    -- Fields to store as persistent cookies (JSON array)
    cookie_fields TEXT DEFAULT '["email", "phone"]',
    -- Fingerprint fields configuration (JSON array of field combinations for matching)
    -- e.g., [["email"], ["full_name", "school", "age"]]
    fingerprint_fields TEXT DEFAULT '[["email"]]',
    -- Thank you configuration
    thank_you_title TEXT DEFAULT 'Thank you!',
    thank_you_message TEXT DEFAULT 'Your response has been recorded.',
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
    -- Step completion message
    completion_title TEXT,
    completion_message TEXT,
    completion_image_url TEXT,
    -- Whether to show completion message after this step
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
    field_type TEXT NOT NULL, -- text, email, phone, number, select, radio, checkbox, textarea, date, hidden
    label TEXT NOT NULL,
    placeholder TEXT,
    help_text TEXT,
    -- Validation rules (JSON)
    validation TEXT DEFAULT '{}',
    -- Options for select/radio/checkbox (JSON array)
    options TEXT,
    -- Field behavior
    is_required INTEGER DEFAULT 0,
    is_identifier INTEGER DEFAULT 0, -- Used for user matching/fingerprinting
    store_in_cookie INTEGER DEFAULT 0, -- Persist in browser cookie
    -- Display order within step
    display_order INTEGER DEFAULT 0,
    -- Conditional display (JSON) - show only if other field matches condition
    conditional TEXT,
    -- Default value
    default_value TEXT,
    -- Metadata
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (step_id) REFERENCES form_steps(id) ON DELETE CASCADE,
    FOREIGN KEY (form_id) REFERENCES forms(id) ON DELETE CASCADE
);

-- User profiles - unified user data across all forms
CREATE TABLE IF NOT EXISTS user_profiles (
    id TEXT PRIMARY KEY,
    -- Fingerprints for matching (JSON object with different fingerprint types)
    fingerprints TEXT DEFAULT '{}',
    -- Known user data (JSON object with all collected field values)
    known_data TEXT DEFAULT '{}',
    -- First seen information
    first_seen_at TEXT DEFAULT CURRENT_TIMESTAMP,
    first_form_id TEXT,
    first_device_info TEXT,
    first_geo_info TEXT,
    -- Last activity
    last_seen_at TEXT DEFAULT CURRENT_TIMESTAMP,
    last_form_id TEXT,
    last_device_info TEXT,
    last_geo_info TEXT,
    -- Statistics
    total_forms_completed INTEGER DEFAULT 0,
    total_interactions INTEGER DEFAULT 0,
    -- Metadata
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);

-- User sessions - individual browsing sessions
CREATE TABLE IF NOT EXISTS user_sessions (
    id TEXT PRIMARY KEY,
    user_profile_id TEXT,
    -- Session state
    is_complete INTEGER DEFAULT 0,
    completed_fields TEXT DEFAULT '[]', -- JSON array of completed identifier fields
    -- Current session data (JSON object)
    session_data TEXT DEFAULT '{}',
    -- Device and geo info from Cloudflare headers
    ip_address TEXT,
    user_agent TEXT,
    country TEXT,
    city TEXT,
    region TEXT,
    timezone TEXT,
    device_type TEXT, -- mobile, tablet, desktop
    browser TEXT,
    os TEXT,
    -- Timestamps
    started_at TEXT DEFAULT CURRENT_TIMESTAMP,
    last_activity_at TEXT DEFAULT CURRENT_TIMESTAMP,
    expires_at TEXT,
    -- Metadata
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_profile_id) REFERENCES user_profiles(id) ON DELETE SET NULL
);

-- Form responses - completed form submissions
CREATE TABLE IF NOT EXISTS form_responses (
    id TEXT PRIMARY KEY,
    form_id TEXT NOT NULL,
    user_session_id TEXT,
    user_profile_id TEXT,
    -- Response status
    status TEXT DEFAULT 'in_progress', -- in_progress, completed, abandoned
    current_step INTEGER DEFAULT 1,
    total_steps INTEGER,
    -- Completion timestamps
    started_at TEXT DEFAULT CURRENT_TIMESTAMP,
    completed_at TEXT,
    -- Device/geo info at submission
    ip_address TEXT,
    country TEXT,
    city TEXT,
    device_type TEXT,
    -- Metadata
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (form_id) REFERENCES forms(id) ON DELETE CASCADE,
    FOREIGN KEY (user_session_id) REFERENCES user_sessions(id) ON DELETE SET NULL,
    FOREIGN KEY (user_profile_id) REFERENCES user_profiles(id) ON DELETE SET NULL
);

-- Field responses - individual field values within a form response
CREATE TABLE IF NOT EXISTS field_responses (
    id TEXT PRIMARY KEY,
    form_response_id TEXT NOT NULL,
    form_field_id TEXT NOT NULL,
    step_id TEXT NOT NULL,
    field_name TEXT NOT NULL,
    field_value TEXT,
    -- Metadata
    submitted_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (form_response_id) REFERENCES form_responses(id) ON DELETE CASCADE,
    FOREIGN KEY (form_field_id) REFERENCES form_fields(id) ON DELETE CASCADE,
    FOREIGN KEY (step_id) REFERENCES form_steps(id) ON DELETE CASCADE
);

-- QR scan events - every time a QR code is scanned
CREATE TABLE IF NOT EXISTS scan_events (
    id TEXT PRIMARY KEY,
    form_id TEXT NOT NULL,
    step_number INTEGER NOT NULL,
    user_session_id TEXT,
    user_profile_id TEXT,
    -- Is this a new or returning user
    is_new_user INTEGER DEFAULT 1,
    is_new_session INTEGER DEFAULT 1,
    -- Device and geo info
    ip_address TEXT,
    user_agent TEXT,
    country TEXT,
    city TEXT,
    region TEXT,
    timezone TEXT,
    device_type TEXT,
    browser TEXT,
    os TEXT,
    -- Referrer info
    referrer TEXT,
    -- Timestamp
    scanned_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (form_id) REFERENCES forms(id) ON DELETE CASCADE,
    FOREIGN KEY (user_session_id) REFERENCES user_sessions(id) ON DELETE SET NULL,
    FOREIGN KEY (user_profile_id) REFERENCES user_profiles(id) ON DELETE SET NULL
);

-- User fingerprints index - for fast matching
CREATE TABLE IF NOT EXISTS user_fingerprints (
    id TEXT PRIMARY KEY,
    user_profile_id TEXT NOT NULL,
    fingerprint_type TEXT NOT NULL, -- e.g., "email", "name_school_age"
    fingerprint_hash TEXT NOT NULL,
    -- Original values used to generate this fingerprint (encrypted JSON)
    source_values TEXT,
    -- Metadata
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
CREATE INDEX IF NOT EXISTS idx_user_sessions_profile ON user_sessions(user_profile_id);
CREATE INDEX IF NOT EXISTS idx_user_sessions_complete ON user_sessions(is_complete);
CREATE INDEX IF NOT EXISTS idx_form_responses_form ON form_responses(form_id);
CREATE INDEX IF NOT EXISTS idx_form_responses_session ON form_responses(user_session_id);
CREATE INDEX IF NOT EXISTS idx_form_responses_profile ON form_responses(user_profile_id);
CREATE INDEX IF NOT EXISTS idx_form_responses_status ON form_responses(status);
CREATE INDEX IF NOT EXISTS idx_field_responses_response ON field_responses(form_response_id);
CREATE INDEX IF NOT EXISTS idx_scan_events_form ON scan_events(form_id);
CREATE INDEX IF NOT EXISTS idx_scan_events_session ON scan_events(user_session_id);
CREATE INDEX IF NOT EXISTS idx_scan_events_date ON scan_events(scanned_at);
CREATE INDEX IF NOT EXISTS idx_user_fingerprints_hash ON user_fingerprints(fingerprint_hash);
CREATE INDEX IF NOT EXISTS idx_user_fingerprints_type ON user_fingerprints(fingerprint_type, fingerprint_hash);
CREATE INDEX IF NOT EXISTS idx_user_fingerprints_profile ON user_fingerprints(user_profile_id);
