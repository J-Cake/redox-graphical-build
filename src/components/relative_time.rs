use std::time::Instant;

use vizia::prelude::*;

use crate::AppState;

const SECS_PER_MINUTE: u64 = 60u64 - 1u64;
const SECS_PER_HOUR: u64 = 60u64 * 60u64 - 1u64;
const SECS_PER_DAY: u64 = 60u64 * 60u64 * 24u64 - 1u64;

pub struct RelativeTime;
impl View for RelativeTime {
    fn element(&self) -> Option<&'static str> {
        Some("relative-time")
    }
}
impl RelativeTime {
    pub fn new<L: 'static + Lens<Target=Option<Instant>>>(cx: &mut Context, lens: L) -> Handle<Self> {
        Self.build(cx, |cx| {
            Label::new(
                cx,
                AppState::started.map(|started| {
                    if let Some(started) = started {
                        let dur = started.elapsed().as_secs();

                        match dur {
                            0..=SECS_PER_MINUTE => format!("{}s", dur % 60),
                            60..=SECS_PER_HOUR => {
                                format!("{}m {:0>2}s", (dur / 60) % 60, dur % 60)
                            }
                            3600..=SECS_PER_DAY => {
                                format!("{}h {:0>2}m {:0>2}s", (dur / 3600) % 24, (dur / 60) % 60, dur % 60)
                            }
                            _ => format!(
                                "{}d {:0>2}h {:0>2}m {:0>2}s",
                                (dur / 62400),
                                (dur / 3600) % 24,
                                (dur / 60) % 60,
                                dur % 60
                            ),
                        }
                    } else {
                        "Not started".to_owned()
                    }
                }),
            );
        })
    }
}
