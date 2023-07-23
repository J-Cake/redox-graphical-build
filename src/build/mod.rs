use std::thread::Thread;

use vizia::icons::*;
use vizia::prelude::*;

use crate::AppEvent;
use crate::AppState;
use crate::components::*;

mod build;
mod config;

pub use config::*;

#[derive(Lens)]
pub struct Builder {
    config: config::BuildConfig,
}

impl View for Builder {
    fn element(&self) -> Option<&'static str> {
        Some(":root")
    }
}

impl Builder {
    pub fn new(cx: &mut Context, config: config::BuildConfig) -> Handle<Self> {
        View::build(Self {
            config: config.clone()
        }, cx, |cx| {            
            Binding::new(cx, AppState::started, |cx, started| {
                if let Some(started) = started.get(cx) {
                    let started: Instant = started;
                    VStack::new(cx, |cx| {
                        HStack::new(cx, |cx| {
                            Button::new(cx, |e| e.emit(AppEvent::BuildStarted(false)), |cx| {
                                HStack::new(cx, |cx| {
                                    Label::new(cx, ICON_PLAYER_STOP);
                                    Label::new(cx, "Stop");
                                })
                            })
                            .class("danger");

                            Button::new(cx, |_event| {}, |cx| {
                                HStack::new(cx, |cx| {
                                    Label::new(cx, ICON_PENCIL);
                                    Label::new(cx, "Edit");
                                })
                            })
                            .class("secondary")
                            .disabled(true);
                        })
                        .height(Auto);

                        Table::with_widths(cx, ["Status", "Job", "Elapsed"], |cx, index, next_width| {
                            Label::new(cx, "Hello World")
                                .width(next_width());
                            Label::new(cx, "Doin' something")
                                .width(next_width());
                            Label::new(cx, "Bye World")
                                .width(next_width());
                            
                            index >= 5
                        }, vec![Units::Auto, Units::Stretch(1.0), Units::Auto])
                            .class("field");
                        HStack::new(cx, |cx| {
                            RelativeTime::new(cx, AppState::started);
                            Element::new(cx).width(Units::Stretch(1.0));

                            Label::new(cx, Builder::config.map(|i| String::from(&i.name)));
                        })
                        .id("status-bar");
                    });
                } else {
                    VStack::new(cx, |cx| {
                        HStack::new(cx, |cx| {
                            Button::new(
                                cx,
                                |e| e.emit(AppEvent::BuildStarted(true)),
                                |cx| {
                                    HStack::new(cx, |cx| {
                                        Label::new(cx, ICON_PLAYER_PLAY);
                                        Label::new(cx, "Build");
                                    })
                                },
                            )
                            .class("primary");

                            Button::new(
                                cx,
                                |_event| {},
                                |cx| {
                                    HStack::new(cx, |cx| {
                                        Label::new(cx, ICON_PENCIL);
                                        Label::new(cx, "Edit");
                                    })
                                },
                            )
                            .class("secondary");
                        })
                        .height(Auto);

                        VStack::new(cx, |cx| {
                            Label::new(cx, "No build running")
                            .height(Units::Stretch(1.0));
                        })
                        .child_space(Stretch(1.0))
                        .col_between(Pixels(20.0));

                        HStack::new(cx, |cx| {
                            RelativeTime::new(cx, AppState::started);
                            Element::new(cx).width(Units::Stretch(1.0));

                            Label::new(cx, Builder::config.map(|i| String::from(&i.name)));
                        })
                        .id("status-bar");
                    });
                }
            });
        })
    }
}
