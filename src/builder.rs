use vizia::prelude::*;

pub struct Builder {
    config: crate::config::BuildConfig
}

impl View for Builder {
    fn element(&self) -> Option<&'static str> {
        Some(":root")
    }
}

impl Builder {
    pub fn new(cx: &mut Context, config: crate::config::BuildConfig) -> Handle<Self> {
        Self { config }.build(cx, |cx| {
            Label::new(cx, "Hello World!");
        })
    }
}