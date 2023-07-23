use std::rc::Rc;
use vizia::{prelude::*, ICON_CHEVRON_DOWN};

struct Row (Rc<Vec<Units>>);
impl Row {
    fn new<Body>(cx: &mut Context, mut body: Body, widths: Rc<Vec<Units>>) -> Handle<Self> where Body: FnMut(&mut Context) {
        let this = Self (widths)
            .build(cx, |cx| {
                (body)(cx)
                
                
            })
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

#[derive(Lens)]
pub struct Table {
    headers: Vec<String>,
    column_widths: Rc<Vec<Units>>,
    sorting: usize
}

impl Table {
    pub fn new<'a, Headers, Body>(cx: &mut Context, headers: Headers, mut body: Body) -> Handle<Self>
    where
        Headers: IntoIterator<Item=&'a str>,
        Body: 'static + FnMut(&mut Context, usize, &mut dyn FnMut() -> Units) -> bool,
    {
        let headers = headers.into_iter().map(|i| i.to_owned()).collect::<Vec<String>>();
        let widths = Rc::new(vec![Units::Percentage(100.0 / headers.len() as f32); headers.len()]);
        
        Self { 
            headers: headers.clone(),
            column_widths: Rc::clone(&widths),
            sorting: 0
        }
            .build(cx, move |cx| {
                let headers = headers;
                let widths = widths;
                
                Row::new(cx, |cx| {
                    for i in headers.iter() {
                        HStack::new(cx, |cx| {
                            Label::new(cx, i)
                                .width(Units::Stretch(1.0))
                                .left(Units::Pixels(4.0))
                                .right(Units::Pixels(2.0));
                            Label::new(cx, ICON_CHEVRON_DOWN)
                                .left(Units::Pixels(2.0))
                                .right(Units::Pixels(4.0));
                        })
                            .class("table-header-column");
                    }
                }, Rc::clone(&widths))
                    .class("table-header");
                
                ScrollView::new(cx, 0.0, 0.0, false, true, move |cx| {
                    let mut index = 0usize;                    
                    let mut should_break = false;
                    while !should_break {
                        let cols = Rc::clone(&widths);
                        let mut cols = cols.iter().cloned();
                        
                        let row = Row::new(cx, |cx| {
                            should_break = body(cx, index, &mut || cols.next().unwrap_or(Units::Pixels(0.0)));
                        }, Rc::clone(&widths));
                        
                        if index % 2 == 1  {
                            row.class("odd");
                        }
                        
                        index += 1;
                    }
                })
                    .class("table-contents");
            })
    }
}

impl View for Table {
    fn element(&self) -> Option<&'static str> {
        Some("table")
    }
}
