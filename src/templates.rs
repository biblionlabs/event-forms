use crate::models::*;

// ============================================================================
// Template files loaded at compile time
// ============================================================================
const TEMPLATE_NOT_FOUND: &str = include_str!("../templates/not_found.html");
const TEMPLATE_THANK_YOU: &str = include_str!("../templates/thank_you.html");
const TEMPLATE_STEP_COMPLETE: &str = include_str!("../templates/step_complete.html");
const TEMPLATE_FORM: &str = include_str!("../templates/form.html");

// ============================================================================
// Dashboard HTML (large static template kept inline for now)
// ============================================================================
/// Generate the main dashboard HTML
pub fn dashboard_html() -> String {
    include_str!("../templates/dashboard.html").to_string()
}

// ============================================================================
// Form HTML
// ============================================================================
/// Generate form HTML for public viewing
pub fn form_html(form: &Form, step: &FormStep, fields: &[FormField], total_steps: i32, known_data: &serde_json::Value) -> String {
    let fields_html = fields.iter().map(|f| render_field(f, known_data)).collect::<Vec<_>>().join("\n");
    let progress_percent = ((step.step_number as f64 / total_steps as f64) * 100.0) as i32;

    let logo_html = form.logo_url.as_ref()
        .map(|u| format!(r#"<img src="{}" alt="Logo" style="max-height:60px;margin-bottom:1rem;">"#, html_escape(u)))
        .unwrap_or_default();

    let step_desc_html = step.description.as_ref()
        .map(|d| format!(r#"<p class="step-description">{}</p>"#, html_escape(d)))
        .unwrap_or_default();

    let submit_text = if step.step_number == total_steps {
        "Enviar"
    } else {
        "Continuar"
    };

    TEMPLATE_FORM
        .replace("{{FORM_NAME}}", &html_escape(&form.name))
        .replace("{{STEP_NUMBER}}", &step.step_number.to_string())
        .replace("{{TOTAL_STEPS}}", &total_steps.to_string())
        .replace("{{PROGRESS}}", &progress_percent.to_string())
        .replace("{{PRIMARY_COLOR}}", &html_escape(&form.primary_color))
        .replace("{{BG_COLOR}}", &html_escape(&form.background_color))
        .replace("{{LOGO}}", &logo_html)
        .replace("{{STEP_TITLE}}", &html_escape(&step.title))
        .replace("{{STEP_DESCRIPTION}}", &step_desc_html)
        .replace("{{SLUG}}", &html_escape(&form.slug))
        .replace("{{FIELDS}}", &fields_html)
        .replace("{{SUBMIT_TEXT}}", submit_text)
}

// ============================================================================
// Thank You HTML
// ============================================================================
/// Generate thank you page HTML
pub fn thank_you_html(form: &Form, step: Option<&FormStep>) -> String {
    let (title, message, image_url) = if let Some(s) = step {
        if s.show_completion_message {
            (
                s.completion_title.clone().unwrap_or_else(|| form.thank_you_title.clone()),
                s.completion_message.clone().unwrap_or_else(|| form.thank_you_message.clone()),
                s.completion_image_url.clone().or_else(|| form.thank_you_image_url.clone())
            )
        } else {
            (form.thank_you_title.clone(), form.thank_you_message.clone(), form.thank_you_image_url.clone())
        }
    } else {
        (form.thank_you_title.clone(), form.thank_you_message.clone(), form.thank_you_image_url.clone())
    };

    let image_html = image_url
        .map(|u| format!(r#"<img src="{}" alt="" style="max-width:200px;margin-bottom:1.5rem;">"#, html_escape(&u)))
        .unwrap_or_default();

    TEMPLATE_THANK_YOU
        .replace("{{TITLE}}", &html_escape(&title))
        .replace("{{MESSAGE}}", &html_escape(&message))
        .replace("{{PRIMARY_COLOR}}", &html_escape(&form.primary_color))
        .replace("{{BG_COLOR}}", &html_escape(&form.background_color))
        .replace("{{IMAGE}}", &image_html)
}

// ============================================================================
// Step Complete HTML
// ============================================================================
/// Generate step completion page HTML (intermediate thank you)
pub fn step_complete_html(form: &Form, step: &FormStep, next_step: i32) -> String {
    let title = step.completion_title.as_deref().unwrap_or("Paso completado");
    let message = step.completion_message.as_deref().unwrap_or("Continua con el siguiente paso.");
    let image_html = step.completion_image_url.as_ref()
        .map(|u| format!(r#"<img src="{}" alt="" style="max-width:200px;margin-bottom:1.5rem;">"#, html_escape(u)))
        .unwrap_or_default();

    TEMPLATE_STEP_COMPLETE
        .replace("{{TITLE}}", &html_escape(title))
        .replace("{{MESSAGE}}", &html_escape(message))
        .replace("{{PRIMARY_COLOR}}", &html_escape(&form.primary_color))
        .replace("{{BG_COLOR}}", &html_escape(&form.background_color))
        .replace("{{SLUG}}", &html_escape(&form.slug))
        .replace("{{NEXT_STEP}}", &next_step.to_string())
        .replace("{{IMAGE}}", &image_html)
}

// ============================================================================
// 404 HTML
// ============================================================================
/// Generate 404 page
pub fn not_found_html() -> String {
    TEMPLATE_NOT_FOUND.to_string()
}

// ============================================================================
// Field Rendering
// ============================================================================
fn render_field(field: &FormField, known_data: &serde_json::Value) -> String {
    let value = known_data.get(&field.field_name)
        .and_then(|v| v.as_str())
        .or(field.default_value.as_deref())
        .unwrap_or("");

    let pre_filled_class = if !value.is_empty() && known_data.get(&field.field_name).is_some() {
        "pre-filled"
    } else {
        ""
    };

    let required_html = if field.is_required { r#"<span class="required">*</span>"# } else { "" };
    let help_html = field.help_text.as_ref()
        .map(|h| format!(r#"<div class="help-text">{}</div>"#, html_escape(h)))
        .unwrap_or_default();

    match &field.field_type {
        FieldType::Text | FieldType::Email | FieldType::Phone | FieldType::Number | FieldType::Date => {
            let input_type = match &field.field_type {
                FieldType::Email => "email",
                FieldType::Phone => "tel",
                FieldType::Number => "number",
                FieldType::Date => "date",
                _ => "text"
            };

            let mut attrs = Vec::new();
            if field.is_required { attrs.push("required".to_string()); }
            if let Some(min) = field.validation.min_length {
                attrs.push(format!("minlength=\"{}\"", min));
            }
            if let Some(max) = field.validation.max_length {
                attrs.push(format!("maxlength=\"{}\"", max));
            }
            if let Some(min) = field.validation.min_value {
                attrs.push(format!("min=\"{}\"", min));
            }
            if let Some(max) = field.validation.max_value {
                attrs.push(format!("max=\"{}\"", max));
            }
            if let Some(pattern) = &field.validation.pattern {
                attrs.push(format!("pattern=\"{}\"", html_escape(pattern)));
                attrs.push(format!("data-pattern=\"{}\"", html_escape(pattern)));
                attrs.push(format!("data-pattern-msg=\"{}\"",
                    html_escape(field.validation.pattern_message.as_deref().unwrap_or("Formato invalido"))));
            }

            format!(r#"
                <div class="form-group">
                    <label>{label} {required}</label>
                    <input type="{input_type}" name="{name}" value="{value}" placeholder="{placeholder}" class="{pre_filled}" {attrs}>
                    {help}
                </div>
            "#,
                label = html_escape(&field.label),
                required = required_html,
                input_type = input_type,
                name = html_escape(&field.field_name),
                value = html_escape(value),
                placeholder = html_escape(field.placeholder.as_deref().unwrap_or("")),
                pre_filled = pre_filled_class,
                attrs = attrs.join(" "),
                help = help_html
            )
        }
        FieldType::Textarea => {
            format!(r#"
                <div class="form-group">
                    <label>{label} {required}</label>
                    <textarea name="{name}" placeholder="{placeholder}" rows="4" class="{pre_filled}" {req}>{value}</textarea>
                    {help}
                </div>
            "#,
                label = html_escape(&field.label),
                required = required_html,
                name = html_escape(&field.field_name),
                placeholder = html_escape(field.placeholder.as_deref().unwrap_or("")),
                value = html_escape(value),
                pre_filled = pre_filled_class,
                req = if field.is_required { "required" } else { "" },
                help = help_html
            )
        }
        FieldType::Select => {
            let options_html = field.options.as_ref().map(|opts| {
                opts.iter().map(|o| {
                    let selected = if o.value == value { " selected" } else { "" };
                    format!(r#"<option value="{}" {}>{}</option>"#, html_escape(&o.value), selected, html_escape(&o.label))
                }).collect::<Vec<_>>().join("\n")
            }).unwrap_or_default();

            format!(r#"
                <div class="form-group">
                    <label>{label} {required}</label>
                    <select name="{name}" class="{pre_filled}" {req}>
                        <option value="">Seleccionar...</option>
                        {options}
                    </select>
                    {help}
                </div>
            "#,
                label = html_escape(&field.label),
                required = required_html,
                name = html_escape(&field.field_name),
                options = options_html,
                pre_filled = pre_filled_class,
                req = if field.is_required { "required" } else { "" },
                help = help_html
            )
        }
        FieldType::Radio => {
            let options_html = field.options.as_ref().map(|opts| {
                opts.iter().map(|o| {
                    let checked = if o.value == value { " checked" } else { "" };
                    format!(r#"
                        <label class="radio-item">
                            <input type="radio" name="{}" value="{}" {}{}>
                            {}
                        </label>
                    "#, html_escape(&field.field_name), html_escape(&o.value), checked, if field.is_required { " required" } else { "" }, html_escape(&o.label))
                }).collect::<Vec<_>>().join("\n")
            }).unwrap_or_default();

            format!(r#"
                <div class="form-group">
                    <label>{label} {required}</label>
                    <div class="radio-group">{options}</div>
                    {help}
                </div>
            "#,
                label = html_escape(&field.label),
                required = required_html,
                options = options_html,
                help = help_html
            )
        }
        FieldType::Checkbox => {
            if field.options.is_some() {
                let options_html = field.options.as_ref().map(|opts| {
                    opts.iter().map(|o| {
                        format!(r#"
                            <label class="checkbox-item">
                                <input type="checkbox" name="{}[]" value="{}">
                                {}
                            </label>
                        "#, html_escape(&field.field_name), html_escape(&o.value), html_escape(&o.label))
                    }).collect::<Vec<_>>().join("\n")
                }).unwrap_or_default();

                format!(r#"
                    <div class="form-group">
                        <label>{label} {required}</label>
                        <div class="checkbox-group">{options}</div>
                        {help}
                    </div>
                "#,
                    label = html_escape(&field.label),
                    required = required_html,
                    options = options_html,
                    help = help_html
                )
            } else {
                let checked = if value == "true" || value == "1" { " checked" } else { "" };
                format!(r#"
                    <div class="form-group">
                        <label class="checkbox-item">
                            <input type="checkbox" name="{name}" value="true" {checked} {req}>
                            {label}
                        </label>
                        {help}
                    </div>
                "#,
                    name = html_escape(&field.field_name),
                    label = html_escape(&field.label),
                    checked = checked,
                    req = if field.is_required { "required" } else { "" },
                    help = help_html
                )
            }
        }
        FieldType::Hidden => {
            format!(r#"<input type="hidden" name="{}" value="{}">"#, html_escape(&field.field_name), html_escape(value))
        }
    }
}

// ============================================================================
// Utilities
// ============================================================================
/// Simple HTML escaping
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&#39;")
}
