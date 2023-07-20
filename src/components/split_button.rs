use vizia::prelude::*;

pub struct SplitButton {
    action: Option<Box<dyn Fn(&mut EventContext)>>,
    menu: Option<Submenu>
}

impl SplitButton {
    /// Creates a new split button.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # let cx = &mut Context::default();
    /// #
    /// Button::new(cx, |_| {}, |cx| Label::new(cx, "Text"));
    /// ```
    pub fn new<A, C, V>(cx: &mut Context, action: A, content: C, menu: Submenu) -> Handle<Self>
    where
        A: 'static + Fn(&mut EventContext),
        C: FnOnce(&mut Context) -> Handle<V>,
        V: 'static + View,
    {
        Self { action: Some(Box::new(action)), menu: Some(menu) }
            .build(cx, move |cx| {
                (content)(cx).hoverable(false).class("inner");
            })
            .role(Role::Button)
            .default_action_verb(DefaultActionVerb::Click)
            .cursor(CursorIcon::Hand)
            .navigable(true)
    }
}

impl View for SplitButton {
    fn element(&self) -> Option<&'static str> {
        Some("button")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::PressDown { mouse } => {
                if *mouse {
                    cx.capture()
                }
                cx.focus();
            }

            WindowEvent::Press { .. } => {
                if meta.target == cx.current() {
                    if let Some(callback) = &self.action {
                        (callback)(cx);
                    }
                }
            }

            WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                cx.release();
            }

            WindowEvent::ActionRequest(action) => match action.action {
                Action::Default => {
                    if let Some(callback) = &self.action {
                        (callback)(cx);
                    }
                }

                _ => {}
            },

            _ => {}
        });
    }
}
