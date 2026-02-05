use serde_json::json;
use worker::{event, Context, Env, Request, Response, Result, Router};

mod auth;
mod crypto;
mod db;
mod models;
mod session;
mod templates;

use auth::require_auth;
use db::Database;
use models::*;
use session::{extract_cookie_data, extract_request_info, SessionManager};
use templates::*;

#[event(start)]
fn start() {
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(|info: &std::panic::PanicHookInfo| {
        worker::console_error!("{info}")
    }));
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        // ====================================================================
        // Public Form Routes
        // ====================================================================
        .get_async("/f/:slug/:step", handle_form_view)
        .post_async("/f/:slug/:step/submit", handle_form_submit)
        .get_async("/f/:slug/:step/complete", handle_step_complete)
        .get_async("/f/:slug/complete", handle_form_complete)

        // ====================================================================
        // Dashboard (protected with basic auth)
        // ====================================================================
        .get_async("/admin", handle_dashboard)
        .get_async("/admin/", handle_dashboard)

        // ====================================================================
        // API Routes (protected with basic auth)
        // ====================================================================
        // Forms
        .get_async("/api/forms", handle_list_forms)
        .post_async("/api/forms", handle_create_form)
        .get_async("/api/forms/:id", handle_get_form)
        .put_async("/api/forms/:id", handle_update_form)
        .delete_async("/api/forms/:id", handle_delete_form)
        .get_async("/api/forms/:id/stats", handle_form_stats)
        .get_async("/api/forms/:id/responses", handle_form_responses)

        // Steps
        .get_async("/api/forms/:form_id/steps", handle_list_steps)
        .post_async("/api/forms/:form_id/steps", handle_create_step)
        .get_async("/api/steps/:id", handle_get_step)
        .put_async("/api/steps/:id", handle_update_step)
        .delete_async("/api/steps/:id", handle_delete_step)

        // Fields
        .get_async("/api/steps/:step_id/fields", handle_list_fields)
        .post_async("/api/steps/:step_id/fields", handle_create_field)
        .get_async("/api/fields/:id", handle_get_field)
        .put_async("/api/fields/:id", handle_update_field)
        .delete_async("/api/fields/:id", handle_delete_field)

        // Users
        .get_async("/api/users", handle_list_users)
        .get_async("/api/users/:id/history", handle_user_history)

        // Schema initialization
        .post_async("/api/init", handle_init_schema)

        // ====================================================================
        // Home redirect
        // ====================================================================
        .get_async("/", |_, _| async move {
            Response::redirect(worker::Url::parse("/admin")?)
        })

        .run(req, env)
        .await
}

// ============================================================================
// Helper Functions
// ============================================================================

fn get_env_var(env: &Env, name: &str, default: &str) -> String {
    env.var(name).map(|v| v.to_string()).unwrap_or_else(|_| default.to_string())
}

fn get_db(env: &Env) -> Result<Database> {
    let d1 = env.d1("DB")?;
    Ok(Database::new(d1))
}

fn get_session_manager(env: &Env) -> SessionManager {
    let encryption_key = get_env_var(env, "ENCRYPTION_KEY", "default-key-change-me!!");
    let cookie_domain = get_env_var(env, "COOKIE_DOMAIN", "");
    SessionManager::new(encryption_key, cookie_domain)
}

async fn check_auth(req: &Request, env: &Env) -> Option<Response> {
    let username = get_env_var(env, "ADMIN_USERNAME", "admin");
    let password = get_env_var(env, "ADMIN_PASSWORD", "admin");
    require_auth(req, &username, &password).await
}

fn json_response<T: serde::Serialize>(data: T) -> Result<Response> {
    let body = serde_json::to_string(&ApiResponse::success(data))?;
    let headers = worker::Headers::new();
    headers.set("Content-Type", "application/json")?;
    Ok(Response::ok(body)?.with_headers(headers))
}

fn json_error(message: &str, status: u16) -> Result<Response> {
    let body = serde_json::to_string(&ApiResponse::<()>::error(message))?;
    let headers = worker::Headers::new();
    headers.set("Content-Type", "application/json")?;
    Ok(Response::ok(body)?.with_headers(headers).with_status(status))
}

fn html_response(html: String) -> Result<Response> {
    Response::from_html(html)
}

fn html_response_with_cookies(html: String, cookies: Vec<String>) -> Result<Response> {
    let mut response = Response::from_html(html)?;
    let headers = response.headers_mut();
    for cookie in cookies {
        headers.append("Set-Cookie", &cookie)?;
    }
    Ok(response)
}

// ============================================================================
// Dashboard Handler
// ============================================================================

async fn handle_dashboard(req: Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    if let Some(auth_response) = check_auth(&req, &ctx.env).await {
        return Ok(auth_response);
    }

    html_response(dashboard_html())
}

// ============================================================================
// Schema Initialization
// ============================================================================

async fn handle_init_schema(req: Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    if let Some(auth_response) = check_auth(&req, &ctx.env).await {
        return Ok(auth_response);
    }

    let db = get_db(&ctx.env)?;
    db.init_schema().await?;

    json_response(json!({"message": "Schema initialized"}))
}

// ============================================================================
// Form CRUD Handlers
// ============================================================================

async fn handle_list_forms(req: Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    if let Some(auth_response) = check_auth(&req, &ctx.env).await {
        return Ok(auth_response);
    }

    let db = get_db(&ctx.env)?;
    let forms = db.list_forms().await?;
    json_response(forms)
}

async fn handle_create_form(mut req: Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    if let Some(auth_response) = check_auth(&req, &ctx.env).await {
        return Ok(auth_response);
    }

    let body: CreateFormRequest = req.json().await?;
    let db = get_db(&ctx.env)?;

    // Check if slug already exists
    if db.get_form_by_slug(&body.slug).await?.is_some() {
        return json_error("Slug already exists", 400);
    }

    let form = db.create_form(body).await?;
    json_response(form)
}

async fn handle_get_form(req: Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    if let Some(auth_response) = check_auth(&req, &ctx.env).await {
        return Ok(auth_response);
    }

    let id = ctx.param("id").unwrap();
    let db = get_db(&ctx.env)?;

    match db.get_form(id).await? {
        Some(form) => json_response(form),
        None => json_error("Form not found", 404),
    }
}

async fn handle_update_form(mut req: Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    if let Some(auth_response) = check_auth(&req, &ctx.env).await {
        return Ok(auth_response);
    }

    let id = ctx.param("id").unwrap();
    let body: UpdateFormRequest = req.json().await?;
    let db = get_db(&ctx.env)?;

    // Check slug uniqueness if changing
    if let Some(ref new_slug) = body.slug {
        if let Some(existing) = db.get_form_by_slug(new_slug).await? {
            if existing.id != *id {
                return json_error("Slug already exists", 400);
            }
        }
    }

    match db.update_form(id, body).await? {
        Some(form) => json_response(form),
        None => json_error("Form not found", 404),
    }
}

async fn handle_delete_form(req: Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    if let Some(auth_response) = check_auth(&req, &ctx.env).await {
        return Ok(auth_response);
    }

    let id = ctx.param("id").unwrap();
    let db = get_db(&ctx.env)?;

    if db.delete_form(id).await? {
        json_response(json!({"deleted": true}))
    } else {
        json_error("Form not found", 404)
    }
}

async fn handle_form_stats(req: Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    if let Some(auth_response) = check_auth(&req, &ctx.env).await {
        return Ok(auth_response);
    }

    let id = ctx.param("id").unwrap();
    let db = get_db(&ctx.env)?;

    let stats = db.get_form_stats(id).await?;
    json_response(stats)
}

async fn handle_form_responses(req: Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    if let Some(auth_response) = check_auth(&req, &ctx.env).await {
        return Ok(auth_response);
    }

    let id = ctx.param("id").unwrap();
    let db = get_db(&ctx.env)?;

    let responses = db.list_responses(id, 100, 0).await?;
    json_response(responses)
}

// ============================================================================
// Step CRUD Handlers
// ============================================================================

async fn handle_list_steps(req: Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    if let Some(auth_response) = check_auth(&req, &ctx.env).await {
        return Ok(auth_response);
    }

    let form_id = ctx.param("form_id").unwrap();
    let db = get_db(&ctx.env)?;

    let steps = db.list_steps(form_id).await?;
    json_response(steps)
}

async fn handle_create_step(mut req: Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    if let Some(auth_response) = check_auth(&req, &ctx.env).await {
        return Ok(auth_response);
    }

    let form_id = ctx.param("form_id").unwrap();
    let body: CreateStepRequest = req.json().await?;
    let db = get_db(&ctx.env)?;

    let step = db.create_step(form_id, body).await?;
    json_response(step)
}

async fn handle_get_step(req: Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    if let Some(auth_response) = check_auth(&req, &ctx.env).await {
        return Ok(auth_response);
    }

    let id = ctx.param("id").unwrap();
    let db = get_db(&ctx.env)?;

    match db.get_step(id).await? {
        Some(step) => json_response(step),
        None => json_error("Step not found", 404),
    }
}

async fn handle_update_step(mut req: Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    if let Some(auth_response) = check_auth(&req, &ctx.env).await {
        return Ok(auth_response);
    }

    let id = ctx.param("id").unwrap();
    let body: UpdateStepRequest = req.json().await?;
    let db = get_db(&ctx.env)?;

    match db.update_step(id, body).await? {
        Some(step) => json_response(step),
        None => json_error("Step not found", 404),
    }
}

async fn handle_delete_step(req: Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    if let Some(auth_response) = check_auth(&req, &ctx.env).await {
        return Ok(auth_response);
    }

    let id = ctx.param("id").unwrap();
    let db = get_db(&ctx.env)?;

    if db.delete_step(id).await? {
        json_response(json!({"deleted": true}))
    } else {
        json_error("Step not found", 404)
    }
}

// ============================================================================
// Field CRUD Handlers
// ============================================================================

async fn handle_list_fields(req: Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    if let Some(auth_response) = check_auth(&req, &ctx.env).await {
        return Ok(auth_response);
    }

    let step_id = ctx.param("step_id").unwrap();
    let db = get_db(&ctx.env)?;

    let fields = db.list_fields(step_id).await?;
    json_response(fields)
}

async fn handle_create_field(mut req: Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    if let Some(auth_response) = check_auth(&req, &ctx.env).await {
        return Ok(auth_response);
    }

    let step_id = ctx.param("step_id").unwrap();
    let body: CreateFieldRequest = req.json().await?;
    let db = get_db(&ctx.env)?;

    // Get step to find form_id
    let step = db.get_step(step_id).await?;
    let step = match step {
        Some(s) => s,
        None => return json_error("Step not found", 404),
    };

    let field = db.create_field(step_id, &step.form_id, body).await?;
    json_response(field)
}

async fn handle_get_field(req: Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    if let Some(auth_response) = check_auth(&req, &ctx.env).await {
        return Ok(auth_response);
    }

    let id = ctx.param("id").unwrap();
    let db = get_db(&ctx.env)?;

    match db.get_field(id).await? {
        Some(field) => json_response(field),
        None => json_error("Field not found", 404),
    }
}

async fn handle_update_field(mut req: Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    if let Some(auth_response) = check_auth(&req, &ctx.env).await {
        return Ok(auth_response);
    }

    let id = ctx.param("id").unwrap();
    let body: UpdateFieldRequest = req.json().await?;
    let db = get_db(&ctx.env)?;

    match db.update_field(id, body).await? {
        Some(field) => json_response(field),
        None => json_error("Field not found", 404),
    }
}

async fn handle_delete_field(req: Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    if let Some(auth_response) = check_auth(&req, &ctx.env).await {
        return Ok(auth_response);
    }

    let id = ctx.param("id").unwrap();
    let db = get_db(&ctx.env)?;

    if db.delete_field(id).await? {
        json_response(json!({"deleted": true}))
    } else {
        json_error("Field not found", 404)
    }
}

// ============================================================================
// User Handlers
// ============================================================================

async fn handle_list_users(req: Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    if let Some(auth_response) = check_auth(&req, &ctx.env).await {
        return Ok(auth_response);
    }

    let db = get_db(&ctx.env)?;
    let users = db.list_users(100, 0).await?;
    json_response(users)
}

async fn handle_user_history(req: Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    if let Some(auth_response) = check_auth(&req, &ctx.env).await {
        return Ok(auth_response);
    }

    let id = ctx.param("id").unwrap();
    let db = get_db(&ctx.env)?;

    let history = db.get_user_history(id).await?;
    json_response(history)
}

// ============================================================================
// Public Form Handlers
// ============================================================================

async fn handle_form_view(req: Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    let slug = ctx.param("slug").unwrap();
    let step_number: i32 = ctx.param("step").unwrap().parse().unwrap_or(1);

    let db = get_db(&ctx.env)?;
    let session_manager = get_session_manager(&ctx.env);

    // Get form
    let form = match db.get_form_by_slug(slug).await? {
        Some(f) if f.is_active => f,
        _ => return html_response(not_found_html()),
    };

    // Get step
    let step = match db.get_step_by_number(&form.id, step_number).await? {
        Some(s) => s,
        None => return html_response(not_found_html()),
    };

    // Get total steps
    let total_steps = db.count_steps(&form.id).await?;

    // Get fields for this step
    let fields = db.list_fields(&step.id).await?;

    // Extract session info from cookies
    let encryption_key = get_env_var(&ctx.env, "ENCRYPTION_KEY", "default-key");
    let (session_id, fingerprint, known_fields) = extract_cookie_data(&req, &encryption_key);

    // Extract request info
    let request_info = extract_request_info(&req);

    // Determine if this is a new session/user
    let is_new_session = session_id.is_none();
    let session_id = session_id.unwrap_or_else(|| session_manager.create_session_id());

    // Try to find existing user profile
    let mut user_profile_id: Option<String> = None;
    let mut is_new_user = true;
    let mut known_data = known_fields.clone();

    // Try to match by fingerprint
    if let Some(fp) = &fingerprint {
        if let Some(profile_id) = db.find_profile_by_fingerprint(fp).await? {
            user_profile_id = Some(profile_id.clone());
            is_new_user = false;

            // Get user's known data
            if let Some(profile) = db.get_user_profile(&profile_id).await? {
                known_data = session_manager.merge_known_data(&known_data, &profile.known_data);
            }
        }
    }

    // Check existing session
    if let Some(existing_session) = db.get_user_session(&session_id).await? {
        if let Some(pid) = existing_session.user_profile_id {
            user_profile_id = Some(pid.clone());
            is_new_user = false;

            if let Some(profile) = db.get_user_profile(&pid).await? {
                known_data = session_manager.merge_known_data(&known_data, &profile.known_data);
            }
        }
        known_data = session_manager.merge_known_data(&known_data, &existing_session.session_data);
    } else {
        // Create new session
        db.create_user_session(&session_id, user_profile_id.as_deref(), &request_info).await?;
    }

    // Record scan event
    db.record_scan_event(
        &form.id,
        step_number,
        Some(&session_id),
        user_profile_id.as_deref(),
        is_new_user,
        is_new_session,
        &request_info,
    ).await?;

    // Build response with cookies
    let html = form_html(&form, &step, &fields, total_steps, &known_data);
    let cookies = session_manager.build_response_cookies(&session_id, fingerprint.as_deref(), Some(&known_data));

    html_response_with_cookies(html, cookies)
}

async fn handle_form_submit(mut req: Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    let slug = ctx.param("slug").unwrap();
    let step_number: i32 = ctx.param("step").unwrap().parse().unwrap_or(1);

    let db = get_db(&ctx.env)?;
    let session_manager = get_session_manager(&ctx.env);
    let encryption_key = get_env_var(&ctx.env, "ENCRYPTION_KEY", "default-key");

    // Get form
    let form = match db.get_form_by_slug(slug).await? {
        Some(f) if f.is_active => f,
        _ => return json_error("Form not found", 404),
    };

    // Get step
    let step = match db.get_step_by_number(&form.id, step_number).await? {
        Some(s) => s,
        None => return json_error("Step not found", 404),
    };

    // Get total steps
    let total_steps = db.count_steps(&form.id).await?;

    // Get fields for this step
    let fields = db.list_fields(&step.id).await?;

    // Parse submitted data
    let body: SubmitStepRequest = req.json().await?;
    let submitted_fields = body.fields;

    // Extract session info
    let (session_id, fingerprint, known_fields) = extract_cookie_data(&req, &encryption_key);
    let request_info = extract_request_info(&req);

    let session_id = session_id.unwrap_or_else(|| session_manager.create_session_id());

    // Get or create session
    let session = match db.get_user_session(&session_id).await? {
        Some(s) => s,
        None => db.create_user_session(&session_id, None, &request_info).await?,
    };

    // Merge submitted data with existing session data
    let mut merged_data = session_manager.merge_known_data(&session.session_data, &submitted_fields);
    merged_data = session_manager.merge_known_data(&merged_data, &known_fields);

    // Get or create form response
    let response = match db.get_active_response(&form.id, &session_id).await? {
        Some(r) => r,
        None => db.create_form_response(&form.id, &session_id, session.user_profile_id.as_deref(), total_steps, &request_info).await?,
    };

    // Save field responses
    for field in &fields {
        if let Some(value) = submitted_fields.get(&field.field_name) {
            let value_str = value.as_str().unwrap_or("");
            db.save_field_response(&response.id, &field.id, &step.id, &field.field_name, value_str).await?;

            // Update merged data
            if !value_str.is_empty() {
                merged_data[&field.field_name] = serde_json::Value::String(value_str.to_string());
            }
        }
    }

    // Get identifier fields for fingerprinting (from field-level config)
    let identifier_fields = db.list_identifier_fields(&form.id).await?;
    let fingerprint_configs: Vec<Vec<String>> = identifier_fields
        .iter()
        .map(|f| vec![f.field_name.clone()])
        .collect();

    // Generate fingerprints based on current data
    let fingerprints = session_manager.generate_fingerprints(&merged_data, &fingerprint_configs);

    // Try to find existing user by fingerprint
    let mut user_profile_id = session.user_profile_id.clone();
    let mut new_fingerprint: Option<String> = fingerprint.clone();

    for (_fp_type, fp_hash) in &fingerprints {
        if let Some(profile_id) = db.find_profile_by_fingerprint(fp_hash).await? {
            // Found existing user!
            user_profile_id = Some(profile_id.clone());

            // Link session and response to this profile
            db.link_session_to_profile(&session_id, &profile_id).await?;
            db.link_response_to_profile(&response.id, &profile_id).await?;

            // Update profile with new data
            db.update_user_profile_data(&profile_id, &merged_data, &request_info, &form.id).await?;

            new_fingerprint = Some(fp_hash.clone());
            break;
        }
    }

    // If no existing profile found, create one
    if user_profile_id.is_none() && !fingerprints.is_empty() {
        let new_profile_id = session_manager.create_user_id();
        db.create_user_profile(&new_profile_id, &request_info, &form.id).await?;

        // Store all fingerprints
        for (fp_type, fp_hash) in &fingerprints {
            db.create_fingerprint(&new_profile_id, fp_type, fp_hash, None).await?;
        }

        // Update profile with data
        db.update_user_profile_data(&new_profile_id, &merged_data, &request_info, &form.id).await?;

        // Link session and response
        db.link_session_to_profile(&session_id, &new_profile_id).await?;
        db.link_response_to_profile(&response.id, &new_profile_id).await?;

        user_profile_id = Some(new_profile_id);
        new_fingerprint = fingerprints.first().map(|(_, h)| h.clone());
    }

    // Session is complete when user has a profile with identifier fields filled
    let is_session_complete = user_profile_id.is_some() && !fingerprints.is_empty();

    // Update session
    db.update_session_data(&session_id, &merged_data, is_session_complete).await?;

    // Get cookie fields to store (from field-level config)
    let cookie_fields = db.list_cookie_fields(&form.id).await?;
    let cookie_field_names: Vec<String> = cookie_fields.iter().map(|f| f.field_name.clone()).collect();
    let cookie_data = session_manager.get_cookie_fields(&merged_data, &cookie_field_names);

    // Build response cookies
    let cookies = session_manager.build_response_cookies(&session_id, new_fingerprint.as_deref(), Some(&cookie_data));

    // Determine redirect
    let is_last_step = step_number == total_steps;

    if is_last_step {
        // Complete the response
        db.complete_response(&response.id).await?;

        // Increment user's completed forms
        if let Some(pid) = &user_profile_id {
            db.increment_forms_completed(pid).await?;
        }

        // Update response progress
        db.update_response_progress(&response.id, step_number + 1).await?;

        // Return redirect to completion page
        let redirect_url = format!("/f/{}/complete", form.slug);

        let body = serde_json::to_string(&json!({
            "success": true,
            "data": {
                "redirect": redirect_url,
                "completed": true
            }
        }))?;

        let mut response = Response::ok(body)?;
        let headers = response.headers_mut();
        headers.set("Content-Type", "application/json")?;
        for cookie in cookies {
            headers.append("Set-Cookie", &cookie)?;
        }
        Ok(response)
    } else {
        // Update response progress
        db.update_response_progress(&response.id, step_number + 1).await?;

        // Check if step has completion message
        let redirect_url = if step.show_completion_message {
            format!("/f/{}/{}/complete", form.slug, step_number)
        } else {
            format!("/f/{}/{}", form.slug, step_number + 1)
        };

        let body = serde_json::to_string(&json!({
            "success": true,
            "data": {
                "redirect": redirect_url,
                "next_step": step_number + 1
            }
        }))?;

        let mut response = Response::ok(body)?;
        let headers = response.headers_mut();
        headers.set("Content-Type", "application/json")?;
        for cookie in cookies {
            headers.append("Set-Cookie", &cookie)?;
        }
        Ok(response)
    }
}

async fn handle_step_complete(_req: Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    let slug = ctx.param("slug").unwrap();
    let step_number: i32 = ctx.param("step").unwrap().parse().unwrap_or(1);

    let db = get_db(&ctx.env)?;

    // Get form
    let form = match db.get_form_by_slug(slug).await? {
        Some(f) => f,
        None => return html_response(not_found_html()),
    };

    // Get step
    let step = match db.get_step_by_number(&form.id, step_number).await? {
        Some(s) => s,
        None => return html_response(not_found_html()),
    };

    html_response(step_complete_html(&form, &step, step_number + 1))
}

async fn handle_form_complete(_req: Request, ctx: worker::RouteContext<()>) -> Result<Response> {
    let slug = ctx.param("slug").unwrap();

    let db = get_db(&ctx.env)?;

    // Get form
    let form = match db.get_form_by_slug(slug).await? {
        Some(f) => f,
        None => return html_response(not_found_html()),
    };

    html_response(thank_you_html(&form, None))
}
