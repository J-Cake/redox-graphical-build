use vizia::{prelude::*, ICON_CHEVRON_DOWN};

type WidthList = Vec<Width>;
type Width = Units;

struct Row<Columns>
where
    Columns: Lens<Target=Vec<Column>>
{ 
    columns: Columns 
}

impl<Columns> Row<Columns>
where
    Columns: 'static + Lens<Target=Vec<Column>>
{
    fn new<Body>(cx: &mut Context, columns: Columns, body: Body) -> Handle<Self>
    where
        Body: FnMut(&mut Context),
    {
        View::build(Self { columns }, cx, |cx| {
            
        })
            .role(Role::Row)
            .layout_type(LayoutType::Row)
    }
}

impl<Columns> View for Row<Columns>
where
    Columns: 'static + Lens<Target=Vec<Column>> {
    fn element(&self) -> Option<&'static str> {
        Some("table-row")
    }
}

pub enum TableEvent {
    ResizeColumn { column: usize, new_size: Units },
    Sort { column: usize },
    MoveColumn { column: usize, index: usize },
}

#[derive(Debug, Clone, Data)]
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
                println!("Resizing column {} to {:?}px", column, &new_size);
                if let Some(column) = self.columns.get_mut(column) {
                    column.width = new_size;
                    cx.needs_relayout();
                }
            }
            Some(_) => todo!(),
        };
    }
}

pub struct TableCell {}

impl View for TableCell {
    fn element(&self) -> Option<&'static str> {
        Some("table-cell")
    }
}

impl Table {
    pub fn new<'a, Headers, Columns, Body>(cx: &mut Context, headers: Headers, body: Body) -> Handle<Self>
    where
        Headers: IntoIterator<Item = &'a str>,
        Columns: 'static + Lens<Target=Vec<Column>>,
        Body: 'static + Fn(&mut Context, usize) -> Option<Vec<Handle<'static, TableColumn>>>
    {
        View::build(Self {
            columns: headers.into_iter()
                .enumerate()
                .map(|(a, i)| Column {
                    heading: i.to_owned(),
                    sort_index: a,
                    width: Units::Stretch(1.0)
                })
                .collect()
        }, cx, |cx| {
            Row::new(cx, Self::columns, |cx| {
                
            });
            
            ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                
            });
        })
    }
}

struct Splitter<OnUpdate: 'static> where OnUpdate: Fn(&mut EventContext, Units) {
    width: Units,
    is_dragging: bool,
    on_update: OnUpdate
}
impl<OnUpdate: 'static> View for Splitter<OnUpdate> where OnUpdate: Fn(&mut EventContext, Units) {
    fn element(&self) -> Option<&'static str> {
        Some("table-splitter")
    }
    
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                self.is_dragging = true;
                cx.capture();
                cx.lock_cursor_icon();
                cx.focus_with_visibility(false);
                cx.with_current(Entity::root(), |cx| {
                    cx.set_pointer_events(false);
                });
            },
            WindowEvent::MouseMove(x, _y) => {
                if self.is_dragging {
                    let cur_width = match self.width {
                        Units::Pixels(px) => px,
                        _ => 0.0
                    };
                
                    let width = ((cx.bounds().x - cur_width) - (cx.bounds().x - x))
                        .max(0.0);
                    (self.on_update)(cx, Units::Pixels(width))
                }
            },
            WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                self.is_dragging = false;
                cx.focus_with_visibility(false);
                cx.release();
                cx.unlock_cursor_icon();
                cx.with_current(Entity::root(), |cx| {
                    cx.set_pointer_events(true);
                });
            },

            _ => {}
        });
    }
}
impl<OnUpdate: 'static> Splitter<OnUpdate> where OnUpdate: Fn(&mut EventContext, Units) {
    pub fn new(cx: &mut Context, initial_width: Units, on_update: OnUpdate) -> Handle<Self> {
        View::build(Self { width: initial_width, is_dragging: false, on_update }, cx, |_| {})
            .height(Units::Pixels(100.0))
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
