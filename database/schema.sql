-- Event Forms - D1 Database Schema

CREATE TABLE IF NOT EXISTS forms (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT DEFAULT '',
  thank_you_title TEXT DEFAULT 'Gracias',
  thank_you_message TEXT DEFAULT 'Tu respuesta ha sido registrada.',
  thank_you_image_url TEXT DEFAULT '',
  identifier_fields TEXT DEFAULT '[]',
  cookie_fields TEXT DEFAULT '[]',
  session_required_fields TEXT DEFAULT '[]',
  is_active INTEGER DEFAULT 1,
  created_at TEXT DEFAULT (datetime('now')),
  updated_at TEXT DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS form_steps (
  id TEXT PRIMARY KEY,
  form_id TEXT NOT NULL,
  step_number INTEGER NOT NULL,
  title TEXT NOT NULL,
  description TEXT DEFAULT '',
  thank_you_title TEXT DEFAULT '',
  thank_you_message TEXT DEFAULT '',
  thank_you_image_url TEXT DEFAULT '',
  is_final INTEGER DEFAULT 0,
  created_at TEXT DEFAULT (datetime('now')),
  FOREIGN KEY (form_id) REFERENCES forms(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS form_fields (
  id TEXT PRIMARY KEY,
  step_id TEXT NOT NULL,
  form_id TEXT NOT NULL,
  field_key TEXT NOT NULL,
  label TEXT NOT NULL,
  field_type TEXT NOT NULL DEFAULT 'text',
  placeholder TEXT DEFAULT '',
  options TEXT DEFAULT '[]',
  validations TEXT DEFAULT '{}',
  is_required INTEGER DEFAULT 0,
  is_identifier INTEGER DEFAULT 0,
  is_cookie INTEGER DEFAULT 0,
  is_session_required INTEGER DEFAULT 0,
  sort_order INTEGER DEFAULT 0,
  created_at TEXT DEFAULT (datetime('now')),
  FOREIGN KEY (step_id) REFERENCES form_steps(id) ON DELETE CASCADE,
  FOREIGN KEY (form_id) REFERENCES forms(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_profiles (
  id TEXT PRIMARY KEY,
  fingerprint TEXT UNIQUE,
  data TEXT NOT NULL DEFAULT '{}',
  created_at TEXT DEFAULT (datetime('now')),
  updated_at TEXT DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS sessions (
  id TEXT PRIMARY KEY,
  user_profile_id TEXT,
  fingerprint TEXT,
  ip_address TEXT,
  user_agent TEXT,
  country TEXT,
  city TEXT,
  region TEXT,
  device_type TEXT,
  created_at TEXT DEFAULT (datetime('now')),
  last_active_at TEXT DEFAULT (datetime('now')),
  FOREIGN KEY (user_profile_id) REFERENCES user_profiles(id)
);

CREATE TABLE IF NOT EXISTS form_responses (
  id TEXT PRIMARY KEY,
  form_id TEXT NOT NULL,
  session_id TEXT NOT NULL,
  user_profile_id TEXT,
  status TEXT DEFAULT 'in_progress',
  current_step INTEGER DEFAULT 1,
  started_at TEXT DEFAULT (datetime('now')),
  completed_at TEXT,
  FOREIGN KEY (form_id) REFERENCES forms(id),
  FOREIGN KEY (session_id) REFERENCES sessions(id),
  FOREIGN KEY (user_profile_id) REFERENCES user_profiles(id)
);

CREATE TABLE IF NOT EXISTS step_responses (
  id TEXT PRIMARY KEY,
  form_response_id TEXT NOT NULL,
  step_id TEXT NOT NULL,
  step_number INTEGER NOT NULL,
  data TEXT NOT NULL DEFAULT '{}',
  submitted_at TEXT DEFAULT (datetime('now')),
  FOREIGN KEY (form_response_id) REFERENCES form_responses(id) ON DELETE CASCADE,
  FOREIGN KEY (step_id) REFERENCES form_steps(id)
);

CREATE TABLE IF NOT EXISTS scan_events (
  id TEXT PRIMARY KEY,
  form_id TEXT NOT NULL,
  step_number INTEGER NOT NULL,
  session_id TEXT,
  user_profile_id TEXT,
  is_new_user INTEGER DEFAULT 1,
  ip_address TEXT,
  user_agent TEXT,
  country TEXT,
  city TEXT,
  region TEXT,
  timezone TEXT,
  device_type TEXT,
  referrer TEXT,
  scanned_at TEXT DEFAULT (datetime('now')),
  FOREIGN KEY (form_id) REFERENCES forms(id)
);

CREATE INDEX IF NOT EXISTS idx_form_steps_form_id ON form_steps(form_id);
CREATE INDEX IF NOT EXISTS idx_form_steps_number ON form_steps(form_id, step_number);
CREATE INDEX IF NOT EXISTS idx_form_fields_step_id ON form_fields(step_id);
CREATE INDEX IF NOT EXISTS idx_form_fields_form_id ON form_fields(form_id);
CREATE INDEX IF NOT EXISTS idx_user_profiles_fingerprint ON user_profiles(fingerprint);
CREATE INDEX IF NOT EXISTS idx_sessions_fingerprint ON sessions(fingerprint);
CREATE INDEX IF NOT EXISTS idx_sessions_user_profile ON sessions(user_profile_id);
CREATE INDEX IF NOT EXISTS idx_form_responses_form_id ON form_responses(form_id);
CREATE INDEX IF NOT EXISTS idx_form_responses_session ON form_responses(session_id);
CREATE INDEX IF NOT EXISTS idx_form_responses_user ON form_responses(user_profile_id);
CREATE INDEX IF NOT EXISTS idx_step_responses_form_response ON step_responses(form_response_id);
CREATE INDEX IF NOT EXISTS idx_scan_events_form_id ON scan_events(form_id);
CREATE INDEX IF NOT EXISTS idx_scan_events_session ON scan_events(session_id);
CREATE INDEX IF NOT EXISTS idx_scan_events_scanned_at ON scan_events(scanned_at);
