use std::io::{BufWriter, Write};

use serde::Serialize;
use tera::Tera;

use crate::{
    AppState,
    model::{
        ButtonProperty, ButtongroupProperty, CheckboxProperty, Form, NumberProperty, Property,
        RangeProperty, SelectProperty, TextProperty, TextareaProperty,
    },
};

pub fn render_form(
    state: AppState,
    language: impl Into<String>,
    form: Form,
    prefix: impl AsRef<[u8]>,
    suffix: impl AsRef<[u8]>,
) -> String {
    let renderer = Renderer::new(language, state.tera);
    let mut output = BufWriter::new(Vec::new());
    output.write_all(prefix.as_ref()).unwrap();
    for prop in form.props.iter() {
        prop.render(&renderer, &mut output);
    }
    output.write_all(suffix.as_ref()).unwrap();
    String::from_utf8(output.into_inner().unwrap()).unwrap()
}

pub struct Renderer {
    language: String,
    tera: Tera,
}
impl Renderer {
    pub fn new(language: impl Into<String>, tera: Tera) -> Self {
        Self {
            language: language.into(),
            tera,
        }
    }
    fn ctx(&self, property: &impl Serialize) -> tera::Context {
        let mut context = tera::Context::new();
        context.insert("property", property);
        context.insert("language", &self.language);
        context
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
        }
    }
}

macro_rules! impl_render {
    ($name: ident, $template: literal) => {
        impl Render for $name {
            fn render(&self, renderer: &Renderer, output: impl Write) {
                let ctx = renderer.ctx(self);
                renderer.tera.render_to($template, &ctx, output).unwrap()
            }
        }
    };
}

impl_render!(TextareaProperty, "textarea_property.html");
impl_render!(TextProperty, "text_property.html");
impl_render!(CheckboxProperty, "checkbox_property.html");
impl_render!(NumberProperty, "number_property.html");
impl_render!(SelectProperty, "select_property.html");
impl_render!(RangeProperty, "range_property.html");
impl_render!(ButtongroupProperty, "buttongroup_property.html");
impl_render!(ButtonProperty, "button_property.html");
