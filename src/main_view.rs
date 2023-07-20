use std::sync::Arc;
use vizia::prelude::*;

#[derive(Clone)]
pub enum ViewMode {
    FileMode,
    EditConfig(Arc<crate::config::BuildConfig>),
    Build(Arc<crate::config::BuildConfig>)
}

pub struct MainView {
    state: ViewMode
}

impl View for MainView {
    fn element(&self) -> Option<&'static str> {
        Some(":root")
    }
}

impl MainView {
    pub fn new(cx: &mut Context, state: ViewMode) -> Handle<Self> {
        Self { state }.build(cx, |cx| {
            
        })
    }
}