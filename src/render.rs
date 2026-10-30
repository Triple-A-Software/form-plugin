use std::{
    collections::BTreeMap,
    io::{BufWriter, Write},
};

use minijinja::context;
use serde::Serialize;

use crate::{
    AppState,
    captcha::CaptchaType,
    model::{
        ButtonProperty, ButtongroupProperty, CaptchaProperty, CheckboxProperty, DateProperty,
        FileProperty, Form, NumberProperty, Property, RangeProperty, SelectProperty, Settings,
        TextProperty, TextareaProperty,
    },
};

/// a field real visitors never see; bots that fill in every input get their submission discarded
pub const HONEYPOT_FIELD: &str = "_fp_hp";

/// shown by `form_errors.html` for the `error` query param set when a submission is rejected
const ERROR_MESSAGES_DE: [(&str, &str); 7] = [
    (
        "generic",
        "Ihre Anfrage konnte nicht gesendet werden. Bitte versuchen Sie es erneut.",
    ),
    (
        "server_error",
        "Ihre Anfrage konnte wegen eines technischen Fehlers nicht gespeichert werden. Bitte versuchen Sie es später erneut.",
    ),
    (
        "rate_limited",
        "Es wurden zu viele Anfragen gesendet. Bitte versuchen Sie es später erneut.",
    ),
    (
        "upload_rejected",
        "Die Dateien konnten nicht angenommen werden. Bitte prüfen Sie Dateityp, Größe und Anzahl.",
    ),
    (
        "captcha_missing",
        "Bitte bestätigen Sie die Sicherheitsabfrage und senden Sie das Formular erneut.",
    ),
    (
        "captcha_error",
        "Die Sicherheitsabfrage ist fehlgeschlagen. Bitte senden Sie das Formular erneut.",
    ),
    (
        "file_rejected",
        "Mindestens eine Datei wurde bei der Sicherheitsprüfung abgelehnt. Die Anfrage wurde nicht übermittelt.",
    ),
];
const ERROR_MESSAGES_EN: [(&str, &str); 7] = [
    (
        "generic",
        "Your request could not be sent. Please try again.",
    ),
    (
        "server_error",
        "Your request could not be saved due to a technical error. Please try again later.",
    ),
    (
        "rate_limited",
        "Too many requests were sent. Please try again later.",
    ),
    (
        "upload_rejected",
        "The files could not be accepted. Please check their type, size and number.",
    ),
    (
        "captcha_missing",
        "Please complete the security check and submit the form again.",
    ),
    (
        "captcha_error",
        "The security check failed. Please submit the form again.",
    ),
    (
        "file_rejected",
        "At least one file was rejected by the security scan. Your request was not sent.",
    ),
];

pub fn render_form(
    state: AppState,
    language: impl Into<String>,
    form: Form,
    prefix: impl AsRef<[u8]>,
    suffix: impl AsRef<[u8]>,
    settings: Settings,
) -> String {
    let language = language.into();
    let messages = if language.starts_with("de") {
        ERROR_MESSAGES_DE
    } else {
        ERROR_MESSAGES_EN
    };
    let renderer = Renderer::new(language, state.environment);
    let mut output = BufWriter::new(Vec::new());
    output.write_all(prefix.as_ref()).unwrap();
    renderer
        .environment
        .get_template("form_errors.html")
        .unwrap()
        .render_to_write(
            context!(form_id => form.id, messages => BTreeMap::from(messages)),
            &mut output,
        )
        .unwrap();
    write!(
        output,
        r#"<div aria-hidden="true" style="position:absolute;left:-10000px;width:1px;height:1px;overflow:hidden"><label>Leave empty <input type="text" name="{HONEYPOT_FIELD}" tabindex="-1" autocomplete="off"></label></div>"#
    )
    .unwrap();
    for prop in form.props.iter() {
        if let Property::Captcha(_) = prop {
            // if the property is a captcha we need to replace its data with the data from the settings and the
            // form itself
            if let (Some(captcha_type), Some(captcha_sitekey)) =
                (&settings.captcha_provider, &settings.captcha_sitekey)
            {
                CaptchaProperty {
                    captcha_class: match captcha_type {
                        CaptchaType::HCaptcha => "h-captcha",
                        CaptchaType::ReCaptcha => "g-recaptcha",
                        CaptchaType::Turnstile => "cf-turnstile",
                    }
                    .to_string(),
                    captcha_type: *captcha_type,
                    sitekey: captcha_sitekey.to_string(),
                    callback: form.captcha_callback.clone(),
                    expired_callback: form.captcha_expired_callback.clone(),
                }
                .render(&renderer, &mut output);
            }
        } else {
            prop.render(&renderer, &mut output);
        }
    }
    output.write_all(form.additional_html.as_bytes()).unwrap();
    output.write_all(suffix.as_ref()).unwrap();
    String::from_utf8(output.into_inner().unwrap()).unwrap()
}

pub struct Renderer<'src> {
    language: String,
    environment: minijinja::Environment<'src>,
}
impl<'src> Renderer<'src> {
    pub fn new(language: impl Into<String>, environment: minijinja::Environment<'src>) -> Self {
        Self {
            language: language.into(),
            environment,
        }
    }
    fn ctx(&self, property: &impl Serialize) -> minijinja::Value {
        context!( property => property, language => &self.language )
    }
}

pub trait Render {
    fn render(&self, renderer: &Renderer, output: impl Write);
}

impl Render for Property {
    fn render(&self, renderer: &Renderer, output: impl Write) {
        match self {
            Property::Textarea(prop) => prop.render(renderer, output),
            Property::Text(prop) => prop.render(renderer, output),
            Property::Checkbox(prop) => prop.render(renderer, output),
            Property::Number(prop) => prop.render(renderer, output),
            Property::Select(prop) => prop.render(renderer, output),
            Property::Range(prop) => prop.render(renderer, output),
            Property::Buttongroup(prop) => prop.render(renderer, output),
            Property::Button(prop) => prop.render(renderer, output),
            Property::Captcha(prop) => prop.render(renderer, output),
            Property::File(prop) => prop.render(renderer, output),
            Property::Date(prop) => prop.render(renderer, output),
        }
    }
}

macro_rules! impl_render {
    ($name: ident, $template: literal) => {
        impl Render for $name {
            fn render(&self, renderer: &Renderer, output: impl Write) {
                let ctx = renderer.ctx(self);
                let template = renderer.environment.get_template($template).unwrap();
                template.render_to_write(&ctx, output).unwrap();
            }
        }
    };
}

impl_render!(FileProperty, "file_property.html");
impl_render!(DateProperty, "date_property.html");
impl_render!(TextareaProperty, "textarea_property.html");
impl_render!(TextProperty, "text_property.html");
impl_render!(CheckboxProperty, "checkbox_property.html");
impl_render!(NumberProperty, "number_property.html");
impl_render!(SelectProperty, "select_property.html");
impl_render!(RangeProperty, "range_property.html");
impl_render!(ButtongroupProperty, "buttongroup_property.html");
impl_render!(ButtonProperty, "button_property.html");
impl_render!(CaptchaProperty, "captcha.html");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_hidden_error_messages_for_the_form() {
        let mut environment = minijinja::Environment::new();
        environment.set_loader(minijinja::path_loader("./templates"));
        let html = environment
            .get_template("form_errors.html")
            .unwrap()
            .render(context!(form_id => 7, messages => BTreeMap::from(ERROR_MESSAGES_DE)))
            .unwrap();
        assert_eq!(
            html.matches(r#"role="alert""#).count(),
            ERROR_MESSAGES_DE.len()
        );
        assert!(html.contains(r#"data-error="file_rejected""#));
        assert!(html.contains("Sicherheitsprüfung abgelehnt"));
        assert!(html.contains(r#"form !== "7""#));
    }
}
