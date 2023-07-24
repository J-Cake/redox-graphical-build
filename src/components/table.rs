use std::ops::Deref;
use std::sync::Mutex;
use std::sync::Arc;
use std::rc::Rc;
use vizia::{prelude::*, ICON_CHEVRON_DOWN};

type WidthList = Vec<Width>;
type Width = Arc<Mutex<Units>>;

struct Row(WidthList);
impl Row {
    fn new<Body, Widths>(cx: &mut Context, body: Body, widths: Widths) -> Handle<Self>
    where
        Body: FnMut(&mut Context),
        Widths: IntoIterator<Item=Width>
    {
        let this = Self(widths.into_iter().collect())
            .build(cx, body)
            .role(Role::Row)
            .layout_type(LayoutType::Row);

        return this;
    }
}

impl View for Row {
    fn element(&self) -> Option<&'static str> {
        Some("table-row")
    }
}

pub enum TableEvent {
    ResizeColumn { column: usize, new_size: Width },
    Sort { column: usize },
    MoveColumn { column: usize, index: usize },
}

#[derive(Debug, Clone, Lens, Data)]
pub struct Column {
    pub heading: String,
    width: Width,
    sort_index: usize,
}

#[derive(Lens)]
pub struct Table {
    columns: Vec<Column>,
}

impl View for Table {
    fn element(&self) -> Option<&'static str> {
        Some("table")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        match event.take() {
            None => {}
            Some(TableEvent::ResizeColumn { column, new_size }) => {
                if let Some(column) = self.columns.get_mut(column) {
                    column.width = new_size;
                    cx.needs_relayout();
                }
            }
            Some(_) => todo!(),
        };
    }
}

impl Table {
    pub fn with_widths<'a, Headers, Body, Widths>(
        cx: &mut Context,
        headers: Headers,
        body: Body,
        widths: Widths,
    ) -> Handle<Self>
    where
        Headers: IntoIterator<Item = &'a str>,
        Body: 'static + Fn(&mut Context, usize, &mut dyn FnMut() -> Units) -> bool,
        Widths: IntoIterator<Item = Units>,
    {
        let widths = widths.into_iter();
        let headers: Vec<String> = headers.into_iter().map(|i| i.to_owned()).collect();

        View::build(
            Self {
                columns: widths
                    .zip(headers.iter().cloned())
                    .enumerate()
                    .map(|(a, (width, heading))| Column {
                        heading,
                        width: Arc::new(Mutex::new(width)),
                        sort_index: a,
                    })
                    .collect(),
            },
            cx,
            move |cx| {
                let body = Rc::new(body);
                let body = Rc::clone(&body);

                Binding::new(cx, Self::columns, move |cx, widths| {
                    let body = Rc::clone(&body);

                    Row::new(cx, |cx| {
                        List::new(cx, Self::columns, |cx, a, column| {
                            let column: Column = column.get(cx);
                            TableHeaderColumn::new(cx, column.heading)
                                .width(*column.width.lock().unwrap());
                            Splitter::new(cx, Column::width);
                        })
                        .layout_type(LayoutType::Row);
                    }, widths.get(cx).iter().map(|i: &Column| Arc::clone(&i.width)))
                    .class("table-header");

                    ScrollView::new(cx, 0.0, 0.0, false, true, move |cx| {})
                        .class("table-contents");
                })
            },
        )
    }

    pub fn new<'a, Headers, Body>(cx: &mut Context, headers: Headers, body: Body) -> Handle<Self>
    where
        Headers: IntoIterator<Item = &'a str>,
        Body: 'static + Fn(&mut Context, usize, &mut dyn FnMut() -> Units) -> bool,
    {
        let mut len = 0usize;
        let mut new_headers = vec![];

        for i in headers.into_iter() {
            new_headers.push(i);
            len += 1;
        }

        let widths = vec![Units::Stretch(1.0); len];
        Table::with_widths(cx, new_headers, body, widths)
    }
}

struct Splitter<L: 'static> where L: Lens<Target=Width> {
    width: L,
    is_dragging: bool
}
impl<L: 'static> View for Splitter<L> where L: 'static + Lens<Target=Width> {
    fn element(&self) -> Option<&'static str> {
        Some("table-splitter")
    }
    
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            WindowEvent::MouseOver => println!("Mouse Over!"),
            WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                println!("Mouse Down");
                self.is_dragging = true;
                cx.capture();
                cx.focus_with_visibility(false);
                cx.with_current(Entity::root(), |cx| {
                    cx.set_pointer_events(false);
                });
            },
            WindowEvent::MouseMove(x, _y) => {
                if self.is_dragging {
                    println!("Mouse Move");
                    let pos_x = cx.cache.get_posx(cx.current());
                    
                }
            },
            WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                println!("Mouse Up");
                self.is_dragging = false;
                cx.focus_with_visibility(false);
                cx.release();
                cx.with_current(Entity::root(), |cx| {
                    cx.set_pointer_events(true);
                });
            }

            _ => {}
        });
    }
}
impl<L: 'static> Splitter<L> where L: 'static + Lens<Target=Width> {
    pub fn new(cx: &mut Context, initial_width: L) -> Handle<Self> {
        View::build(Self { width: initial_width, is_dragging: false }, cx, |cx| {
            Element::new(cx)
                .height(Units::Stretch(1.0))
                .width(Units::Pixels(2.0));
        })
    }
}

#[derive(Lens)]
struct TableHeaderColumn {
    heading: String,
}

impl View for TableHeaderColumn {
    fn element(&self) -> Option<&'static str> {
        Some("table-header-column")
    }
}

impl TableHeaderColumn {
    pub fn new(cx: &mut Context, column: String) -> Handle<Self> {
        View::build(Self { heading: column }, cx, move |cx| {
            HStack::new(cx, |cx| {
                Label::new(cx, Self::heading.map(|i| i.clone()))
                    .width(Units::Stretch(1.0))
                    .left(Units::Pixels(2.0))
                    .right(Units::Pixels(2.0));
                Label::new(cx, ICON_CHEVRON_DOWN)
                    .left(Units::Pixels(2.0))
                    .right(Units::Pixels(2.0));
            }).height(Units::Auto);
        })
    }
}
