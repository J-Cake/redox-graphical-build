use vizia::prelude::*;
use vizia::icons::*;

use crate::components::*;

pub struct Builder {
    config: crate::config::BuildConfig,
}

impl View for Builder {
    fn element(&self) -> Option<&'static str> {
        Some(":root")
    }
}

impl Builder {
    pub fn new(cx: &mut Context, config: crate::config::BuildConfig) -> Handle<Self> {
        Self { config }.build(cx, |cx| {
            VStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Button::new(cx, |event| {}, |cx| HStack::new(cx, |cx| {
                        Label::new(cx, ICON_PLAYER_PLAY);
                        Label::new(cx, "Build");
                    }))
                        .class("primary");
                    
                    Button::new(cx, |event| {
                        
                    }, |cx| HStack::new(cx, |cx| {
                        Label::new(cx, ICON_PENCIL);
                        Label::new(cx, "Edit");
                    }))
                        .class("secondary");
                })
                    .height(Auto);
                
                Table::new(cx, ["Status", "Job", "Elapsed"], |cx, index| {
                    Label::new(cx, "Hello World");
                    Label::new(cx, "Gbye");
                    Label::new(cx, "Test");
                    
                    index < 5
                })
                    .class("field");
            });
        })
    }
}