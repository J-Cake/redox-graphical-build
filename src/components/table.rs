use std::rc::Rc;
use std::cell::RefCell;
use vizia::{prelude::*, ICON_CHEVRON_DOWN};

type WidthList = Rc<Vec<Rc<RefCell<Units>>>>;

struct Row (WidthList);
impl Row {
    fn new<Body>(cx: &mut Context, body: Body, widths: WidthList) -> Handle<Self> where Body: FnMut(&mut Context) {
        let this = Self (widths)
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
    ResizeColumn {
        column: usize,
        new_size: Units
    },
    Sort {
        column: usize
    },
    MoveColumn {
        column: usize,
        index: usize
    }
}

#[derive(Lens)]
pub struct Table {
    headers: Vec<String>,
    column_widths: WidthList,
    sorting: usize,
}

impl View for Table {
    fn element(&self) -> Option<&'static str> {
        Some("table")
    }
    
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        match event.take() {
            None => {},
            Some(TableEvent::ResizeColumn { column, new_size }) => {
                self.column_widths[column].replace(new_size);
                cx.needs_relayout();
            }
            Some(_) => todo!()
        };
    }
}

impl Table {
    pub fn with_widths<'a, Headers, Body, Widths>(cx: &mut Context, headers: Headers, body: Body, widths: Widths) -> Handle<Self> 
    where 
        Headers: IntoIterator<Item=&'a str>,
        Body: 'static + Fn(&mut Context, usize, &mut dyn FnMut() -> Units) -> bool,
        Widths: IntoIterator<Item=Units>
    {
        let widths = Rc::new(widths.into_iter().map(|i| Rc::new(RefCell::new(i))).collect());
        let headers = headers.into_iter().map(|i| i.to_owned()).collect::<Vec<String>>();
        
        View::build(Self { 
            headers: headers.clone(),
            column_widths: Rc::clone(&widths),
            sorting: 0,
        }, cx, move |cx| {
            let headers = headers;
            let body = Rc::new(body);
            let body = Rc::clone(&body);
                            
            Binding::new(cx, Self::column_widths, move |cx, widths| {
                let body = Rc::clone(&body);
                let widths: WidthList = widths.get(cx);
                
                Row::new(cx, |cx| {
                    for (a, i) in headers.iter().enumerate() {
                        let width: Rc<RefCell<Units>> = Rc::clone(&widths[a]);
                        TableHeaderColumn::new(cx, (i.clone(), a), Some(width));
                    }
                }, Rc::clone(&widths))
                    .class("table-header");
                
                let width_list = Rc::clone(&widths);
                ScrollView::new(cx, 0.0, 0.0, false, true, move |cx| {
                    let mut index = 0usize;                    
                    let mut should_break = false;
                    while !should_break {
                        let cols = Rc::clone(&width_list);
                        let mut cols = cols.iter().cloned();
                        
                        let row = Row::new(cx, |cx| {
                            should_break = Rc::clone(&body)(cx, index, &mut || 
                                if let Some(width) = cols.next() {
                                    let width: Rc<RefCell<Units>> = Rc::clone(&width);
                                    let unit = RefCell::borrow(&width).clone();
                                    unit.clone()
                                } else {
                                    Units::Pixels(0.0)
                                });
                        }, Rc::clone(&width_list));
                        
                        if index % 2 == 1  {
                            row.class("odd");
                        }
                        
                        index += 1;
                    }
                })
                    .class("table-contents");
            })
        })
    }
    
    pub fn new<'a, Headers, Body>(cx: &mut Context, headers: Headers, body: Body) -> Handle<Self>
    where
        Headers: IntoIterator<Item=&'a str>,
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
    
    pub fn widths<Iter: IntoIterator<Item=Units>>(&mut self, items: Iter) -> &mut Self {
        self.column_widths = Rc::new(items
            .into_iter()
            .map(|i| Rc::new(RefCell::new(i)))
            .collect());
        return self;
    }
}

#[derive(Lens)]
struct TableHeaderColumn {
    heading: String,
    width: Option<Rc<RefCell<Units>>>,
    column: usize,
    
    is_dragging: bool
}

impl View for TableHeaderColumn {
    fn element(&self) -> Option<&'static str> {
        Some("table-header-column")
    }
    
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        if let Some(width) = &self.width {
            event.map(|e, _| match e {
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
                        width.replace(Units::Pixels((pos_x - x).max(0.0)));
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
}

impl TableHeaderColumn {
    pub fn new(cx: &mut Context, column: (String, usize), width: Option<Rc<RefCell<Units>>>) -> Handle<Self> {
        View::build(Self { 
            heading: column.0, 
            column: column.1, 
            width,
            is_dragging: false 
        }, cx, move |cx| {
            Binding::new(cx, TableHeaderColumn::width, |cx, width| {
                let width = width.get(cx);
                
                HStack::new(cx, |cx| {
                    Label::new(cx, Self::heading.map(|i| i.clone()))
                        .width(Units::Stretch(1.0))
                        .left(Units::Pixels(2.0))
                        .right(Units::Pixels(2.0));
                    Label::new(cx, ICON_CHEVRON_DOWN)
                        .left(Units::Pixels(2.0))
                        .right(Units::Pixels(2.0));
                })
                    .height(Units::Auto)
                    .width(width.map(|width| RefCell::borrow(&width).clone()).unwrap_or(Units::Stretch(1.0)));
            });
        })
    }
}