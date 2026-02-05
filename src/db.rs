use crate::models::*;
use serde_json::json;
use worker::*;

/// Get current timestamp as ISO string
fn now_iso() -> String {
    let date = js_sys::Date::new_0();
    date.to_iso_string().as_string().unwrap_or_else(|| "".to_string())
}

/// Get future timestamp (days from now) as ISO string
fn future_iso(days: i32) -> String {
    let date = js_sys::Date::new_0();
    let ms = date.get_time() + (days as f64 * 24.0 * 60.0 * 60.0 * 1000.0);
    date.set_time(ms);
    date.to_iso_string().as_string().unwrap_or_else(|| "".to_string())
}

/// Database operations for Event Forms
pub struct Database {
    db: D1Database,
}

impl Database {
    pub fn new(db: D1Database) -> Self {
        Database { db }
    }

    /// Initialize database schema
    pub async fn init_schema(&self) -> Result<()> {
        let schema = include_str!("../schema.sql");

        // Split by semicolons and execute each statement
        for statement in schema.split(';') {
            let trimmed = statement.trim();
            if !trimmed.is_empty() && !trimmed.starts_with("--") {
                self.db.exec(trimmed).await?;
            }
        }

        Ok(())
    }

    // ========================================================================
    // Forms CRUD
    // ========================================================================

    pub async fn create_form(&self, req: CreateFormRequest) -> Result<Form> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = now_iso();

        let tags_json = serde_json::to_string(&req.tags)
            .unwrap_or_else(|_| "[]".to_string());

        let stmt = self.db.prepare(
            "INSERT INTO forms (id, name, description, slug, tags, thank_you_title, thank_you_message,
             thank_you_image_url, primary_color, logo_url, background_color, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)"
        );

        stmt.bind(&[
            id.clone().into(),
            req.name.clone().into(),
            req.description.clone().into(),
            req.slug.clone().into(),
            tags_json.into(),
            req.thank_you_title.clone().into(),
            req.thank_you_message.clone().into(),
            req.thank_you_image_url.clone().into(),
            req.primary_color.clone().into(),
            req.logo_url.clone().into(),
            req.background_color.clone().into(),
            now.clone().into(),
            now.clone().into(),
        ])?
        .run()
        .await?;

        Ok(Form {
            id,
            name: req.name,
            description: req.description,
            slug: req.slug,
            is_active: true,
            tags: req.tags,
            thank_you_title: req.thank_you_title,
            thank_you_message: req.thank_you_message,
            thank_you_image_url: req.thank_you_image_url,
            primary_color: req.primary_color,
            logo_url: req.logo_url,
            background_color: req.background_color,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub async fn get_form(&self, id: &str) -> Result<Option<Form>> {
        let stmt = self.db.prepare("SELECT * FROM forms WHERE id = ?1");
        let result = stmt.bind(&[id.into()])?.first::<FormRow>(None).await?;
        Ok(result.map(|r| r.into()))
    }

    pub async fn get_form_by_slug(&self, slug: &str) -> Result<Option<Form>> {
        let stmt = self.db.prepare("SELECT * FROM forms WHERE slug = ?1");
        let result = stmt.bind(&[slug.into()])?.first::<FormRow>(None).await?;
        Ok(result.map(|r| r.into()))
    }

    pub async fn list_forms(&self) -> Result<Vec<Form>> {
        let stmt = self.db.prepare("SELECT * FROM forms ORDER BY created_at DESC");
        let results = stmt.all().await?;
        let rows: Vec<FormRow> = results.results()?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn update_form(&self, id: &str, req: UpdateFormRequest) -> Result<Option<Form>> {
        let existing = self.get_form(id).await?;
        if existing.is_none() {
            return Ok(None);
        }
        let existing = existing.unwrap();

        let now = now_iso();
        let name = req.name.unwrap_or(existing.name);
        let description = req.description.or(existing.description);
        let slug = req.slug.unwrap_or(existing.slug);
        let is_active = req.is_active.unwrap_or(existing.is_active);
        let tags = req.tags.unwrap_or(existing.tags);
        let thank_you_title = req.thank_you_title.unwrap_or(existing.thank_you_title);
        let thank_you_message = req.thank_you_message.unwrap_or(existing.thank_you_message);
        let thank_you_image_url = req.thank_you_image_url.or(existing.thank_you_image_url);
        let primary_color = req.primary_color.unwrap_or(existing.primary_color);
        let logo_url = req.logo_url.or(existing.logo_url);
        let background_color = req.background_color.unwrap_or(existing.background_color);

        let stmt = self.db.prepare(
            "UPDATE forms SET name = ?1, description = ?2, slug = ?3, is_active = ?4, tags = ?5,
             thank_you_title = ?6, thank_you_message = ?7, thank_you_image_url = ?8,
             primary_color = ?9, logo_url = ?10, background_color = ?11, updated_at = ?12
             WHERE id = ?13"
        );

        stmt.bind(&[
            name.clone().into(),
            description.clone().into(),
            slug.clone().into(),
            (is_active as i32).into(),
            serde_json::to_string(&tags).unwrap().into(),
            thank_you_title.clone().into(),
            thank_you_message.clone().into(),
            thank_you_image_url.clone().into(),
            primary_color.clone().into(),
            logo_url.clone().into(),
            background_color.clone().into(),
            now.clone().into(),
            id.into(),
        ])?
        .run()
        .await?;

        Ok(Some(Form {
            id: id.to_string(),
            name,
            description,
            slug,
            is_active,
            tags,
            thank_you_title,
            thank_you_message,
            thank_you_image_url,
            primary_color,
            logo_url,
            background_color,
            created_at: existing.created_at,
            updated_at: now,
        }))
    }

    pub async fn delete_form(&self, id: &str) -> Result<bool> {
        let stmt = self.db.prepare("DELETE FROM forms WHERE id = ?1");
        let result = stmt.bind(&[id.into()])?.run().await?;
        Ok(result.success())
    }

    // ========================================================================
    // Form Steps CRUD
    // ========================================================================

    pub async fn create_step(&self, form_id: &str, req: CreateStepRequest) -> Result<FormStep> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = now_iso();

        // Get next step number
        let count_stmt = self.db.prepare("SELECT COUNT(*) as count FROM form_steps WHERE form_id = ?1");
        let count_result = count_stmt.bind(&[form_id.into()])?.first::<CountRow>(None).await?;
        let step_number = count_result.map(|r| r.count + 1).unwrap_or(1);

        let stmt = self.db.prepare(
            "INSERT INTO form_steps (id, form_id, step_number, title, description,
             completion_title, completion_message, completion_image_url, show_completion_message,
             created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)"
        );

        stmt.bind(&[
            id.clone().into(),
            form_id.into(),
            step_number.into(),
            req.title.clone().into(),
            req.description.clone().into(),
            req.completion_title.clone().into(),
            req.completion_message.clone().into(),
            req.completion_image_url.clone().into(),
            (req.show_completion_message as i32).into(),
            now.clone().into(),
            now.clone().into(),
        ])?
        .run()
        .await?;

        Ok(FormStep {
            id,
            form_id: form_id.to_string(),
            step_number,
            title: req.title,
            description: req.description,
            completion_title: req.completion_title,
            completion_message: req.completion_message,
            completion_image_url: req.completion_image_url,
            show_completion_message: req.show_completion_message,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub async fn get_step(&self, id: &str) -> Result<Option<FormStep>> {
        let stmt = self.db.prepare("SELECT * FROM form_steps WHERE id = ?1");
        let result = stmt.bind(&[id.into()])?.first::<FormStepRow>(None).await?;
        Ok(result.map(|r| r.into()))
    }

    pub async fn get_step_by_number(&self, form_id: &str, step_number: i32) -> Result<Option<FormStep>> {
        let stmt = self.db.prepare("SELECT * FROM form_steps WHERE form_id = ?1 AND step_number = ?2");
        let result = stmt.bind(&[form_id.into(), step_number.into()])?.first::<FormStepRow>(None).await?;
        Ok(result.map(|r| r.into()))
    }

    pub async fn list_steps(&self, form_id: &str) -> Result<Vec<FormStep>> {
        let stmt = self.db.prepare("SELECT * FROM form_steps WHERE form_id = ?1 ORDER BY step_number");
        let results = stmt.bind(&[form_id.into()])?.all().await?;
        let rows: Vec<FormStepRow> = results.results()?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn update_step(&self, id: &str, req: UpdateStepRequest) -> Result<Option<FormStep>> {
        let existing = self.get_step(id).await?;
        if existing.is_none() {
            return Ok(None);
        }
        let existing = existing.unwrap();

        let now = now_iso();
        let title = req.title.unwrap_or(existing.title);
        let description = req.description.or(existing.description);
        let step_number = req.step_number.unwrap_or(existing.step_number);
        let completion_title = req.completion_title.or(existing.completion_title);
        let completion_message = req.completion_message.or(existing.completion_message);
        let completion_image_url = req.completion_image_url.or(existing.completion_image_url);
        let show_completion_message = req.show_completion_message.unwrap_or(existing.show_completion_message);

        let stmt = self.db.prepare(
            "UPDATE form_steps SET title = ?1, description = ?2, step_number = ?3,
             completion_title = ?4, completion_message = ?5, completion_image_url = ?6,
             show_completion_message = ?7, updated_at = ?8 WHERE id = ?9"
        );

        stmt.bind(&[
            title.clone().into(),
            description.clone().into(),
            step_number.into(),
            completion_title.clone().into(),
            completion_message.clone().into(),
            completion_image_url.clone().into(),
            (show_completion_message as i32).into(),
            now.clone().into(),
            id.into(),
        ])?
        .run()
        .await?;

        Ok(Some(FormStep {
            id: id.to_string(),
            form_id: existing.form_id,
            step_number,
            title,
            description,
            completion_title,
            completion_message,
            completion_image_url,
            show_completion_message,
            created_at: existing.created_at,
            updated_at: now,
        }))
    }

    pub async fn delete_step(&self, id: &str) -> Result<bool> {
        let stmt = self.db.prepare("DELETE FROM form_steps WHERE id = ?1");
        let result = stmt.bind(&[id.into()])?.run().await?;
        Ok(result.success())
    }

    pub async fn count_steps(&self, form_id: &str) -> Result<i32> {
        let stmt = self.db.prepare("SELECT COUNT(*) as count FROM form_steps WHERE form_id = ?1");
        let result = stmt.bind(&[form_id.into()])?.first::<CountRow>(None).await?;
        Ok(result.map(|r| r.count).unwrap_or(0))
    }

    // ========================================================================
    // Form Fields CRUD
    // ========================================================================

    pub async fn create_field(&self, step_id: &str, form_id: &str, req: CreateFieldRequest) -> Result<FormField> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = now_iso();

        let validation_json = serde_json::to_string(&req.validation).unwrap_or_else(|_| "{}".to_string());
        let options_json = req.options.as_ref().map(|o| serde_json::to_string(o).unwrap_or_else(|_| "[]".to_string()));
        let conditional_json = req.conditional.as_ref().map(|c| serde_json::to_string(c).unwrap_or_else(|_| "{}".to_string()));

        let stmt = self.db.prepare(
            "INSERT INTO form_fields (id, step_id, form_id, field_name, field_type, label,
             placeholder, help_text, validation, options, is_required, is_identifier,
             store_in_cookie, display_order, conditional, default_value, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)"
        );

        stmt.bind(&[
            id.clone().into(),
            step_id.into(),
            form_id.into(),
            req.field_name.clone().into(),
            req.field_type.clone().into(),
            req.label.clone().into(),
            req.placeholder.clone().into(),
            req.help_text.clone().into(),
            validation_json.into(),
            options_json.into(),
            (req.is_required as i32).into(),
            (req.is_identifier as i32).into(),
            (req.store_in_cookie as i32).into(),
            req.display_order.into(),
            conditional_json.into(),
            req.default_value.clone().into(),
            now.clone().into(),
            now.clone().into(),
        ])?
        .run()
        .await?;

        Ok(FormField {
            id,
            step_id: step_id.to_string(),
            form_id: form_id.to_string(),
            field_name: req.field_name,
            field_type: FieldType::from_str(&req.field_type),
            label: req.label,
            placeholder: req.placeholder,
            help_text: req.help_text,
            validation: req.validation,
            options: req.options,
            is_required: req.is_required,
            is_identifier: req.is_identifier,
            store_in_cookie: req.store_in_cookie,
            display_order: req.display_order,
            conditional: req.conditional,
            default_value: req.default_value,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub async fn get_field(&self, id: &str) -> Result<Option<FormField>> {
        let stmt = self.db.prepare("SELECT * FROM form_fields WHERE id = ?1");
        let result = stmt.bind(&[id.into()])?.first::<FormFieldRow>(None).await?;
        Ok(result.map(|r| r.into()))
    }

    pub async fn list_fields(&self, step_id: &str) -> Result<Vec<FormField>> {
        let stmt = self.db.prepare("SELECT * FROM form_fields WHERE step_id = ?1 ORDER BY display_order");
        let results = stmt.bind(&[step_id.into()])?.all().await?;
        let rows: Vec<FormFieldRow> = results.results()?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn list_form_fields(&self, form_id: &str) -> Result<Vec<FormField>> {
        let stmt = self.db.prepare("SELECT * FROM form_fields WHERE form_id = ?1 ORDER BY display_order");
        let results = stmt.bind(&[form_id.into()])?.all().await?;
        let rows: Vec<FormFieldRow> = results.results()?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn list_identifier_fields(&self, form_id: &str) -> Result<Vec<FormField>> {
        let stmt = self.db.prepare("SELECT * FROM form_fields WHERE form_id = ?1 AND is_identifier = 1 ORDER BY display_order");
        let results = stmt.bind(&[form_id.into()])?.all().await?;
        let rows: Vec<FormFieldRow> = results.results()?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn list_cookie_fields(&self, form_id: &str) -> Result<Vec<FormField>> {
        let stmt = self.db.prepare("SELECT * FROM form_fields WHERE form_id = ?1 AND store_in_cookie = 1 ORDER BY display_order");
        let results = stmt.bind(&[form_id.into()])?.all().await?;
        let rows: Vec<FormFieldRow> = results.results()?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn update_field(&self, id: &str, req: UpdateFieldRequest) -> Result<Option<FormField>> {
        let existing = self.get_field(id).await?;
        if existing.is_none() {
            return Ok(None);
        }
        let existing = existing.unwrap();

        let now = now_iso();
        let field_name = req.field_name.unwrap_or(existing.field_name);
        let field_type = req.field_type.map(|t| FieldType::from_str(&t)).unwrap_or(existing.field_type);
        let label = req.label.unwrap_or(existing.label);
        let placeholder = req.placeholder.or(existing.placeholder);
        let help_text = req.help_text.or(existing.help_text);
        let validation = req.validation.unwrap_or(existing.validation);
        let options = req.options.or(existing.options);
        let is_required = req.is_required.unwrap_or(existing.is_required);
        let is_identifier = req.is_identifier.unwrap_or(existing.is_identifier);
        let store_in_cookie = req.store_in_cookie.unwrap_or(existing.store_in_cookie);
        let display_order = req.display_order.unwrap_or(existing.display_order);
        let conditional = req.conditional.or(existing.conditional);
        let default_value = req.default_value.or(existing.default_value);

        let validation_json = serde_json::to_string(&validation).unwrap_or_else(|_| "{}".to_string());
        let options_json = options.as_ref().map(|o| serde_json::to_string(o).unwrap_or_else(|_| "[]".to_string()));
        let conditional_json = conditional.as_ref().map(|c| serde_json::to_string(c).unwrap_or_else(|_| "{}".to_string()));

        let stmt = self.db.prepare(
            "UPDATE form_fields SET field_name = ?1, field_type = ?2, label = ?3, placeholder = ?4,
             help_text = ?5, validation = ?6, options = ?7, is_required = ?8, is_identifier = ?9,
             store_in_cookie = ?10, display_order = ?11, conditional = ?12, default_value = ?13,
             updated_at = ?14 WHERE id = ?15"
        );

        stmt.bind(&[
            field_name.clone().into(),
            field_type.as_str().into(),
            label.clone().into(),
            placeholder.clone().into(),
            help_text.clone().into(),
            validation_json.into(),
            options_json.into(),
            (is_required as i32).into(),
            (is_identifier as i32).into(),
            (store_in_cookie as i32).into(),
            display_order.into(),
            conditional_json.into(),
            default_value.clone().into(),
            now.clone().into(),
            id.into(),
        ])?
        .run()
        .await?;

        Ok(Some(FormField {
            id: id.to_string(),
            step_id: existing.step_id,
            form_id: existing.form_id,
            field_name,
            field_type,
            label,
            placeholder,
            help_text,
            validation,
            options,
            is_required,
            is_identifier,
            store_in_cookie,
            display_order,
            conditional,
            default_value,
            created_at: existing.created_at,
            updated_at: now,
        }))
    }

    pub async fn delete_field(&self, id: &str) -> Result<bool> {
        let stmt = self.db.prepare("DELETE FROM form_fields WHERE id = ?1");
        let result = stmt.bind(&[id.into()])?.run().await?;
        Ok(result.success())
    }

    // ========================================================================
    // User Profiles
    // ========================================================================

    pub async fn create_user_profile(&self, id: &str, request_info: &RequestInfo, form_id: &str) -> Result<UserProfile> {
        let now = now_iso();

        let stmt = self.db.prepare(
            "INSERT INTO user_profiles (id, known_data, first_seen_at, first_form_id,
             first_device_info, first_geo_info, last_seen_at, last_form_id, last_device_info,
             last_geo_info, total_forms_completed, total_interactions, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)"
        );

        stmt.bind(&[
            id.into(),
            "{}".into(),
            now.clone().into(),
            form_id.into(),
            request_info.to_device_json().into(),
            request_info.to_geo_json().into(),
            now.clone().into(),
            form_id.into(),
            request_info.to_device_json().into(),
            request_info.to_geo_json().into(),
            0.into(),
            1.into(),
            now.clone().into(),
            now.clone().into(),
        ])?
        .run()
        .await?;

        Ok(UserProfile {
            id: id.to_string(),
            known_data: json!({}),
            first_seen_at: now.clone(),
            first_form_id: Some(form_id.to_string()),
            first_device_info: Some(request_info.to_device_json()),
            first_geo_info: Some(request_info.to_geo_json()),
            last_seen_at: now.clone(),
            last_form_id: Some(form_id.to_string()),
            last_device_info: Some(request_info.to_device_json()),
            last_geo_info: Some(request_info.to_geo_json()),
            total_forms_completed: 0,
            total_interactions: 1,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub async fn get_user_profile(&self, id: &str) -> Result<Option<UserProfile>> {
        let stmt = self.db.prepare("SELECT * FROM user_profiles WHERE id = ?1");
        let result = stmt.bind(&[id.into()])?.first::<UserProfileRow>(None).await?;
        Ok(result.map(|r| r.into()))
    }

    pub async fn update_user_profile_data(&self, id: &str, known_data: &serde_json::Value, request_info: &RequestInfo, form_id: &str) -> Result<()> {
        let now = now_iso();

        let stmt = self.db.prepare(
            "UPDATE user_profiles SET known_data = ?1, last_seen_at = ?2, last_form_id = ?3,
             last_device_info = ?4, last_geo_info = ?5, total_interactions = total_interactions + 1,
             updated_at = ?6 WHERE id = ?7"
        );

        stmt.bind(&[
            serde_json::to_string(known_data).unwrap().into(),
            now.clone().into(),
            form_id.into(),
            request_info.to_device_json().into(),
            request_info.to_geo_json().into(),
            now.into(),
            id.into(),
        ])?
        .run()
        .await?;

        Ok(())
    }

    pub async fn increment_forms_completed(&self, id: &str) -> Result<()> {
        let stmt = self.db.prepare(
            "UPDATE user_profiles SET total_forms_completed = total_forms_completed + 1 WHERE id = ?1"
        );
        stmt.bind(&[id.into()])?.run().await?;
        Ok(())
    }

    // ========================================================================
    // User Sessions
    // ========================================================================

    pub async fn create_user_session(&self, id: &str, user_profile_id: Option<&str>, request_info: &RequestInfo) -> Result<UserSession> {
        let now = now_iso();
        let expires = Some(future_iso(365));

        let stmt = self.db.prepare(
            "INSERT INTO user_sessions (id, user_profile_id, is_complete, session_data,
             ip_address, user_agent, country, city, region, timezone, device_type, browser, os,
             started_at, last_activity_at, expires_at, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)"
        );

        stmt.bind(&[
            id.into(),
            user_profile_id.into(),
            0.into(),
            "{}".into(),
            request_info.ip_address.clone().into(),
            request_info.user_agent.clone().into(),
            request_info.country.clone().into(),
            request_info.city.clone().into(),
            request_info.region.clone().into(),
            request_info.timezone.clone().into(),
            request_info.device_type.clone().into(),
            request_info.browser.clone().into(),
            request_info.os.clone().into(),
            now.clone().into(),
            now.clone().into(),
            expires.clone().into(),
            now.clone().into(),
        ])?
        .run()
        .await?;

        Ok(UserSession {
            id: id.to_string(),
            user_profile_id: user_profile_id.map(|s| s.to_string()),
            is_complete: false,
            session_data: json!({}),
            ip_address: request_info.ip_address.clone(),
            user_agent: request_info.user_agent.clone(),
            country: request_info.country.clone(),
            city: request_info.city.clone(),
            region: request_info.region.clone(),
            timezone: request_info.timezone.clone(),
            device_type: request_info.device_type.clone(),
            browser: request_info.browser.clone(),
            os: request_info.os.clone(),
            started_at: now.clone(),
            last_activity_at: now.clone(),
            expires_at: expires,
            created_at: now,
        })
    }

    pub async fn get_user_session(&self, id: &str) -> Result<Option<UserSession>> {
        let stmt = self.db.prepare("SELECT * FROM user_sessions WHERE id = ?1");
        let result = stmt.bind(&[id.into()])?.first::<UserSessionRow>(None).await?;
        Ok(result.map(|r| r.into()))
    }

    pub async fn update_session_data(&self, id: &str, session_data: &serde_json::Value, is_complete: bool) -> Result<()> {
        let now = now_iso();

        let stmt = self.db.prepare(
            "UPDATE user_sessions SET session_data = ?1, is_complete = ?2, last_activity_at = ?3 WHERE id = ?4"
        );

        stmt.bind(&[
            serde_json::to_string(session_data).unwrap().into(),
            (is_complete as i32).into(),
            now.into(),
            id.into(),
        ])?
        .run()
        .await?;

        Ok(())
    }

    pub async fn link_session_to_profile(&self, session_id: &str, profile_id: &str) -> Result<()> {
        let stmt = self.db.prepare("UPDATE user_sessions SET user_profile_id = ?1 WHERE id = ?2");
        stmt.bind(&[profile_id.into(), session_id.into()])?.run().await?;
        Ok(())
    }

    // ========================================================================
    // User Fingerprints
    // ========================================================================

    pub async fn create_fingerprint(&self, user_profile_id: &str, fp_type: &str, fp_hash: &str, source_values: Option<&str>) -> Result<()> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = now_iso();

        let stmt = self.db.prepare(
            "INSERT INTO user_fingerprints (id, user_profile_id, fingerprint_type, fingerprint_hash, source_values, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
        );

        stmt.bind(&[
            id.into(),
            user_profile_id.into(),
            fp_type.into(),
            fp_hash.into(),
            source_values.into(),
            now.into(),
        ])?
        .run()
        .await?;

        Ok(())
    }

    pub async fn find_profile_by_fingerprint(&self, fp_hash: &str) -> Result<Option<String>> {
        let stmt = self.db.prepare("SELECT user_profile_id FROM user_fingerprints WHERE fingerprint_hash = ?1 LIMIT 1");
        let result = stmt.bind(&[fp_hash.into()])?.first::<FingerprintRow>(None).await?;
        Ok(result.map(|r| r.user_profile_id))
    }

    #[allow(dead_code)]
    pub async fn fingerprint_exists(&self, fp_hash: &str) -> Result<bool> {
        let stmt = self.db.prepare("SELECT COUNT(*) as count FROM user_fingerprints WHERE fingerprint_hash = ?1");
        let result = stmt.bind(&[fp_hash.into()])?.first::<CountRow>(None).await?;
        Ok(result.map(|r| r.count > 0).unwrap_or(false))
    }

    // ========================================================================
    // Form Responses
    // ========================================================================

    pub async fn create_form_response(&self, form_id: &str, session_id: &str, profile_id: Option<&str>, total_steps: i32, request_info: &RequestInfo) -> Result<FormResponse> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = now_iso();

        let stmt = self.db.prepare(
            "INSERT INTO form_responses (id, form_id, user_session_id, user_profile_id, status,
             current_step, total_steps, started_at, ip_address, country, city, device_type,
             created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)"
        );

        stmt.bind(&[
            id.clone().into(),
            form_id.into(),
            session_id.into(),
            profile_id.into(),
            "in_progress".into(),
            1.into(),
            total_steps.into(),
            now.clone().into(),
            request_info.ip_address.clone().into(),
            request_info.country.clone().into(),
            request_info.city.clone().into(),
            request_info.device_type.clone().into(),
            now.clone().into(),
            now.clone().into(),
        ])?
        .run()
        .await?;

        Ok(FormResponse {
            id,
            form_id: form_id.to_string(),
            user_session_id: Some(session_id.to_string()),
            user_profile_id: profile_id.map(|s| s.to_string()),
            status: ResponseStatus::InProgress,
            current_step: 1,
            total_steps: Some(total_steps),
            started_at: now.clone(),
            completed_at: None,
            ip_address: request_info.ip_address.clone(),
            country: request_info.country.clone(),
            city: request_info.city.clone(),
            device_type: request_info.device_type.clone(),
            created_at: now.clone(),
            updated_at: now,
        })
    }

    #[allow(dead_code)]
    pub async fn get_form_response(&self, id: &str) -> Result<Option<FormResponse>> {
        let stmt = self.db.prepare("SELECT * FROM form_responses WHERE id = ?1");
        let result = stmt.bind(&[id.into()])?.first::<FormResponseRow>(None).await?;
        Ok(result.map(|r| r.into()))
    }

    pub async fn get_active_response(&self, form_id: &str, session_id: &str) -> Result<Option<FormResponse>> {
        let stmt = self.db.prepare(
            "SELECT * FROM form_responses WHERE form_id = ?1 AND user_session_id = ?2 AND status = 'in_progress' ORDER BY created_at DESC LIMIT 1"
        );
        let result = stmt.bind(&[form_id.into(), session_id.into()])?.first::<FormResponseRow>(None).await?;
        Ok(result.map(|r| r.into()))
    }

    pub async fn update_response_progress(&self, id: &str, current_step: i32) -> Result<()> {
        let now = now_iso();
        let stmt = self.db.prepare("UPDATE form_responses SET current_step = ?1, updated_at = ?2 WHERE id = ?3");
        stmt.bind(&[current_step.into(), now.into(), id.into()])?.run().await?;
        Ok(())
    }

    pub async fn complete_response(&self, id: &str) -> Result<()> {
        let now = now_iso();
        let stmt = self.db.prepare("UPDATE form_responses SET status = 'completed', completed_at = ?1, updated_at = ?2 WHERE id = ?3");
        stmt.bind(&[now.clone().into(), now.into(), id.into()])?.run().await?;
        Ok(())
    }

    pub async fn link_response_to_profile(&self, response_id: &str, profile_id: &str) -> Result<()> {
        let stmt = self.db.prepare("UPDATE form_responses SET user_profile_id = ?1 WHERE id = ?2");
        stmt.bind(&[profile_id.into(), response_id.into()])?.run().await?;
        Ok(())
    }

    // ========================================================================
    // Field Responses
    // ========================================================================

    pub async fn save_field_response(&self, response_id: &str, field_id: &str, step_id: &str, field_name: &str, value: &str) -> Result<()> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = now_iso();

        // Check if response already exists for this field
        let existing = self.db.prepare(
            "SELECT id FROM field_responses WHERE form_response_id = ?1 AND form_field_id = ?2"
        ).bind(&[response_id.into(), field_id.into()])?.first::<IdRow>(None).await?;

        if let Some(existing) = existing {
            // Update existing
            let stmt = self.db.prepare("UPDATE field_responses SET field_value = ?1, submitted_at = ?2 WHERE id = ?3");
            stmt.bind(&[value.into(), now.into(), existing.id.into()])?.run().await?;
        } else {
            // Insert new
            let stmt = self.db.prepare(
                "INSERT INTO field_responses (id, form_response_id, form_field_id, step_id, field_name, field_value, submitted_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"
            );
            stmt.bind(&[
                id.into(),
                response_id.into(),
                field_id.into(),
                step_id.into(),
                field_name.into(),
                value.into(),
                now.into(),
            ])?.run().await?;
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn get_field_responses(&self, response_id: &str) -> Result<Vec<FieldResponse>> {
        let stmt = self.db.prepare("SELECT * FROM field_responses WHERE form_response_id = ?1");
        let results = stmt.bind(&[response_id.into()])?.all().await?;
        let rows: Vec<FieldResponseRow> = results.results()?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    // ========================================================================
    // Scan Events
    // ========================================================================

    pub async fn record_scan_event(&self, form_id: &str, step_number: i32, session_id: Option<&str>, profile_id: Option<&str>, is_new_user: bool, is_new_session: bool, request_info: &RequestInfo) -> Result<()> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = now_iso();

        let stmt = self.db.prepare(
            "INSERT INTO scan_events (id, form_id, step_number, user_session_id, user_profile_id,
             is_new_user, is_new_session, ip_address, user_agent, country, city, region, timezone,
             device_type, browser, os, referrer, scanned_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)"
        );

        stmt.bind(&[
            id.into(),
            form_id.into(),
            step_number.into(),
            session_id.into(),
            profile_id.into(),
            (is_new_user as i32).into(),
            (is_new_session as i32).into(),
            request_info.ip_address.clone().into(),
            request_info.user_agent.clone().into(),
            request_info.country.clone().into(),
            request_info.city.clone().into(),
            request_info.region.clone().into(),
            request_info.timezone.clone().into(),
            request_info.device_type.clone().into(),
            request_info.browser.clone().into(),
            request_info.os.clone().into(),
            request_info.referrer.clone().into(),
            now.into(),
        ])?
        .run()
        .await?;

        Ok(())
    }

    // ========================================================================
    // Analytics
    // ========================================================================

    pub async fn get_form_stats(&self, form_id: &str) -> Result<FormStats> {
        // Get form name
        let form = self.get_form(form_id).await?.unwrap_or_else(|| Form {
            id: form_id.to_string(),
            name: "Unknown".to_string(),
            description: None,
            slug: "".to_string(),
            is_active: false,
            tags: vec![],
            thank_you_title: "".to_string(),
            thank_you_message: "".to_string(),
            thank_you_image_url: None,
            primary_color: "".to_string(),
            logo_url: None,
            background_color: "".to_string(),
            created_at: "".to_string(),
            updated_at: "".to_string(),
        });

        // Total scans
        let total_scans = self.db.prepare("SELECT COUNT(*) as count FROM scan_events WHERE form_id = ?1")
            .bind(&[form_id.into()])?.first::<CountRow>(None).await?.map(|r| r.count as i64).unwrap_or(0);

        // Unique users
        let unique_users = self.db.prepare("SELECT COUNT(DISTINCT user_profile_id) as count FROM scan_events WHERE form_id = ?1 AND user_profile_id IS NOT NULL")
            .bind(&[form_id.into()])?.first::<CountRow>(None).await?.map(|r| r.count as i64).unwrap_or(0);

        // New vs returning
        let new_users = self.db.prepare("SELECT COUNT(*) as count FROM scan_events WHERE form_id = ?1 AND is_new_user = 1")
            .bind(&[form_id.into()])?.first::<CountRow>(None).await?.map(|r| r.count as i64).unwrap_or(0);
        let returning_users = total_scans - new_users;

        // Response stats
        let completed = self.db.prepare("SELECT COUNT(*) as count FROM form_responses WHERE form_id = ?1 AND status = 'completed'")
            .bind(&[form_id.into()])?.first::<CountRow>(None).await?.map(|r| r.count as i64).unwrap_or(0);
        let in_progress = self.db.prepare("SELECT COUNT(*) as count FROM form_responses WHERE form_id = ?1 AND status = 'in_progress'")
            .bind(&[form_id.into()])?.first::<CountRow>(None).await?.map(|r| r.count as i64).unwrap_or(0);
        let abandoned = self.db.prepare("SELECT COUNT(*) as count FROM form_responses WHERE form_id = ?1 AND status = 'abandoned'")
            .bind(&[form_id.into()])?.first::<CountRow>(None).await?.map(|r| r.count as i64).unwrap_or(0);

        let total_responses = completed + in_progress + abandoned;
        let completion_rate = if total_responses > 0 { (completed as f64 / total_responses as f64) * 100.0 } else { 0.0 };

        // Device breakdown
        let mobile = self.db.prepare("SELECT COUNT(*) as count FROM scan_events WHERE form_id = ?1 AND device_type = 'mobile'")
            .bind(&[form_id.into()])?.first::<CountRow>(None).await?.map(|r| r.count as i64).unwrap_or(0);
        let tablet = self.db.prepare("SELECT COUNT(*) as count FROM scan_events WHERE form_id = ?1 AND device_type = 'tablet'")
            .bind(&[form_id.into()])?.first::<CountRow>(None).await?.map(|r| r.count as i64).unwrap_or(0);
        let desktop = self.db.prepare("SELECT COUNT(*) as count FROM scan_events WHERE form_id = ?1 AND device_type = 'desktop'")
            .bind(&[form_id.into()])?.first::<CountRow>(None).await?.map(|r| r.count as i64).unwrap_or(0);

        // Geo breakdown (top 10 countries)
        let geo_rows = self.db.prepare(
            "SELECT country, COUNT(*) as count FROM scan_events WHERE form_id = ?1 AND country IS NOT NULL GROUP BY country ORDER BY count DESC LIMIT 10"
        ).bind(&[form_id.into()])?.all().await?;
        let geo_data: Vec<GeoRow> = geo_rows.results()?;
        let geo_breakdown: Vec<GeoItem> = geo_data.into_iter().map(|r| GeoItem {
            country: r.country.unwrap_or_else(|| "Unknown".to_string()),
            count: r.count
        }).collect();

        // Step funnel
        let steps = self.list_steps(form_id).await?;
        let mut step_funnel = Vec::new();
        for step in &steps {
            let started = self.db.prepare("SELECT COUNT(*) as count FROM scan_events WHERE form_id = ?1 AND step_number = ?2")
                .bind(&[form_id.into(), step.step_number.into()])?.first::<CountRow>(None).await?.map(|r| r.count as i64).unwrap_or(0);
            let completed_step = self.db.prepare("SELECT COUNT(*) as count FROM form_responses WHERE form_id = ?1 AND current_step > ?2")
                .bind(&[form_id.into(), step.step_number.into()])?.first::<CountRow>(None).await?.map(|r| r.count as i64).unwrap_or(0);
            let drop_off = if started > 0 { ((started - completed_step) as f64 / started as f64) * 100.0 } else { 0.0 };

            step_funnel.push(StepFunnelItem {
                step_number: step.step_number,
                step_title: step.title.clone(),
                started,
                completed: completed_step,
                drop_off_rate: drop_off,
            });
        }

        Ok(FormStats {
            form_id: form_id.to_string(),
            form_name: form.name,
            total_scans,
            unique_users,
            new_users,
            returning_users,
            completed_responses: completed,
            in_progress_responses: in_progress,
            abandoned_responses: abandoned,
            completion_rate,
            step_funnel,
            device_breakdown: DeviceBreakdown {
                mobile,
                tablet,
                desktop,
                unknown: total_scans - mobile - tablet - desktop,
            },
            geo_breakdown,
            hourly_scans: vec![], // Would need time-based query
        })
    }

    pub async fn get_user_history(&self, profile_id: &str) -> Result<UserInteractionHistory> {
        let profile = self.get_user_profile(profile_id).await?;

        let profile = profile.ok_or_else(|| worker::Error::RustError("Profile not found".to_string()))?;

        // Get all scan events for this user
        let scan_rows = self.db.prepare(
            "SELECT se.*, f.name as form_name FROM scan_events se
             LEFT JOIN forms f ON se.form_id = f.id
             WHERE se.user_profile_id = ?1
             ORDER BY se.scanned_at DESC LIMIT 100"
        ).bind(&[profile_id.into()])?.all().await?;
        let scans: Vec<ScanEventWithFormRow> = scan_rows.results()?;

        let interactions: Vec<UserInteraction> = scans.into_iter().map(|s| UserInteraction {
            interaction_type: "scan".to_string(),
            form_id: s.form_id,
            form_name: s.form_name.unwrap_or_else(|| "Unknown".to_string()),
            step_number: Some(s.step_number),
            timestamp: s.scanned_at,
            device_type: s.device_type,
            country: s.country,
            city: s.city,
        }).collect();

        Ok(UserInteractionHistory {
            user_profile_id: profile_id.to_string(),
            known_data: profile.known_data,
            total_forms: profile.total_forms_completed,
            total_interactions: profile.total_interactions,
            first_seen: profile.first_seen_at,
            last_seen: profile.last_seen_at,
            interactions,
        })
    }

    pub async fn list_users(&self, limit: i32, offset: i32) -> Result<Vec<UserProfile>> {
        let stmt = self.db.prepare("SELECT * FROM user_profiles ORDER BY last_seen_at DESC LIMIT ?1 OFFSET ?2");
        let results = stmt.bind(&[limit.into(), offset.into()])?.all().await?;
        let rows: Vec<UserProfileRow> = results.results()?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    #[allow(dead_code)]
    pub async fn count_users(&self) -> Result<i32> {
        let stmt = self.db.prepare("SELECT COUNT(*) as count FROM user_profiles");
        let result = stmt.first::<CountRow>(None).await?;
        Ok(result.map(|r| r.count).unwrap_or(0))
    }

    pub async fn list_responses(&self, form_id: &str, limit: i32, offset: i32) -> Result<Vec<FormResponse>> {
        let stmt = self.db.prepare(
            "SELECT * FROM form_responses WHERE form_id = ?1 ORDER BY created_at DESC LIMIT ?2 OFFSET ?3"
        );
        let results = stmt.bind(&[form_id.into(), limit.into(), offset.into()])?.all().await?;
        let rows: Vec<FormResponseRow> = results.results()?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

// ============================================================================
// Row types for D1 deserialization
// ============================================================================

#[derive(Debug, serde::Deserialize)]
struct CountRow {
    count: i32,
}

#[derive(Debug, serde::Deserialize)]
struct IdRow {
    id: String,
}

#[derive(Debug, serde::Deserialize)]
struct FingerprintRow {
    user_profile_id: String,
}

#[derive(Debug, serde::Deserialize)]
struct FormRow {
    id: String,
    name: String,
    description: Option<String>,
    slug: String,
    is_active: i32,
    tags: Option<String>,
    thank_you_title: Option<String>,
    thank_you_message: Option<String>,
    thank_you_image_url: Option<String>,
    primary_color: Option<String>,
    logo_url: Option<String>,
    background_color: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<FormRow> for Form {
    fn from(r: FormRow) -> Self {
        Form {
            id: r.id,
            name: r.name,
            description: r.description,
            slug: r.slug,
            is_active: r.is_active != 0,
            tags: r.tags.and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default(),
            thank_you_title: r.thank_you_title.unwrap_or_else(|| "¡Gracias!".to_string()),
            thank_you_message: r.thank_you_message.unwrap_or_else(|| "Tu respuesta ha sido registrada.".to_string()),
            thank_you_image_url: r.thank_you_image_url,
            primary_color: r.primary_color.unwrap_or_else(|| "#3B82F6".to_string()),
            logo_url: r.logo_url,
            background_color: r.background_color.unwrap_or_else(|| "#F9FAFB".to_string()),
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct FormStepRow {
    id: String,
    form_id: String,
    step_number: i32,
    title: String,
    description: Option<String>,
    completion_title: Option<String>,
    completion_message: Option<String>,
    completion_image_url: Option<String>,
    show_completion_message: i32,
    created_at: String,
    updated_at: String,
}

impl From<FormStepRow> for FormStep {
    fn from(r: FormStepRow) -> Self {
        FormStep {
            id: r.id,
            form_id: r.form_id,
            step_number: r.step_number,
            title: r.title,
            description: r.description,
            completion_title: r.completion_title,
            completion_message: r.completion_message,
            completion_image_url: r.completion_image_url,
            show_completion_message: r.show_completion_message != 0,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct FormFieldRow {
    id: String,
    step_id: String,
    form_id: String,
    field_name: String,
    field_type: String,
    label: String,
    placeholder: Option<String>,
    help_text: Option<String>,
    validation: String,
    options: Option<String>,
    is_required: i32,
    is_identifier: i32,
    store_in_cookie: i32,
    display_order: i32,
    conditional: Option<String>,
    default_value: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<FormFieldRow> for FormField {
    fn from(r: FormFieldRow) -> Self {
        FormField {
            id: r.id,
            step_id: r.step_id,
            form_id: r.form_id,
            field_name: r.field_name,
            field_type: FieldType::from_str(&r.field_type),
            label: r.label,
            placeholder: r.placeholder,
            help_text: r.help_text,
            validation: serde_json::from_str(&r.validation).unwrap_or_default(),
            options: r.options.and_then(|o| serde_json::from_str(&o).ok()),
            is_required: r.is_required != 0,
            is_identifier: r.is_identifier != 0,
            store_in_cookie: r.store_in_cookie != 0,
            display_order: r.display_order,
            conditional: r.conditional.and_then(|c| serde_json::from_str(&c).ok()),
            default_value: r.default_value,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct UserProfileRow {
    id: String,
    known_data: Option<String>,
    first_seen_at: String,
    first_form_id: Option<String>,
    first_device_info: Option<String>,
    first_geo_info: Option<String>,
    last_seen_at: String,
    last_form_id: Option<String>,
    last_device_info: Option<String>,
    last_geo_info: Option<String>,
    total_forms_completed: i32,
    total_interactions: i32,
    created_at: String,
    updated_at: String,
}

impl From<UserProfileRow> for UserProfile {
    fn from(r: UserProfileRow) -> Self {
        UserProfile {
            id: r.id,
            known_data: r.known_data.and_then(|d| serde_json::from_str(&d).ok()).unwrap_or_else(|| json!({})),
            first_seen_at: r.first_seen_at,
            first_form_id: r.first_form_id,
            first_device_info: r.first_device_info,
            first_geo_info: r.first_geo_info,
            last_seen_at: r.last_seen_at,
            last_form_id: r.last_form_id,
            last_device_info: r.last_device_info,
            last_geo_info: r.last_geo_info,
            total_forms_completed: r.total_forms_completed,
            total_interactions: r.total_interactions,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct UserSessionRow {
    id: String,
    user_profile_id: Option<String>,
    is_complete: i32,
    session_data: Option<String>,
    ip_address: Option<String>,
    user_agent: Option<String>,
    country: Option<String>,
    city: Option<String>,
    region: Option<String>,
    timezone: Option<String>,
    device_type: Option<String>,
    browser: Option<String>,
    os: Option<String>,
    started_at: String,
    last_activity_at: String,
    expires_at: Option<String>,
    created_at: String,
}

impl From<UserSessionRow> for UserSession {
    fn from(r: UserSessionRow) -> Self {
        UserSession {
            id: r.id,
            user_profile_id: r.user_profile_id,
            is_complete: r.is_complete != 0,
            session_data: r.session_data.and_then(|d| serde_json::from_str(&d).ok()).unwrap_or_else(|| json!({})),
            ip_address: r.ip_address,
            user_agent: r.user_agent,
            country: r.country,
            city: r.city,
            region: r.region,
            timezone: r.timezone,
            device_type: r.device_type,
            browser: r.browser,
            os: r.os,
            started_at: r.started_at,
            last_activity_at: r.last_activity_at,
            expires_at: r.expires_at,
            created_at: r.created_at,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct FormResponseRow {
    id: String,
    form_id: String,
    user_session_id: Option<String>,
    user_profile_id: Option<String>,
    status: String,
    current_step: i32,
    total_steps: Option<i32>,
    started_at: String,
    completed_at: Option<String>,
    ip_address: Option<String>,
    country: Option<String>,
    city: Option<String>,
    device_type: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<FormResponseRow> for FormResponse {
    fn from(r: FormResponseRow) -> Self {
        FormResponse {
            id: r.id,
            form_id: r.form_id,
            user_session_id: r.user_session_id,
            user_profile_id: r.user_profile_id,
            status: ResponseStatus::from_str(&r.status),
            current_step: r.current_step,
            total_steps: r.total_steps,
            started_at: r.started_at,
            completed_at: r.completed_at,
            ip_address: r.ip_address,
            country: r.country,
            city: r.city,
            device_type: r.device_type,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct FieldResponseRow {
    id: String,
    form_response_id: String,
    form_field_id: String,
    step_id: String,
    field_name: String,
    field_value: Option<String>,
    submitted_at: String,
}

impl From<FieldResponseRow> for FieldResponse {
    fn from(r: FieldResponseRow) -> Self {
        FieldResponse {
            id: r.id,
            form_response_id: r.form_response_id,
            form_field_id: r.form_field_id,
            step_id: r.step_id,
            field_name: r.field_name,
            field_value: r.field_value,
            submitted_at: r.submitted_at,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct GeoRow {
    country: Option<String>,
    count: i64,
}

#[derive(Debug, serde::Deserialize)]
struct ScanEventWithFormRow {
    form_id: String,
    form_name: Option<String>,
    step_number: i32,
    scanned_at: String,
    device_type: Option<String>,
    country: Option<String>,
    city: Option<String>,
}
