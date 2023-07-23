use vizia::prelude::*;

#[derive(Lens)]
pub struct Editor {
    open: bool
}

impl Editor {
    pub fn new(cx: &mut Context, config: crate::build::BuildConfig) -> Self {
        Popup::new(cx, Editor::open, true, |cx| {
            Label::new(cx, "Hello World");
        });
        
        Self {
            open: true
        }
    }
}