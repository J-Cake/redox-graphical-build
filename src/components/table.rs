use std::rc::Rc;
use vizia::{prelude::*, ICON_CHEVRON_DOWN};

struct Row {
    widths: Rc<Vec<Units>>
}

impl Row {
    fn new<Body>(cx: &mut Context, mut body: Body, widths: Rc<Vec<Units>>) -> Handle<Self> where Body: FnMut(&mut Context) {
        let this = Self { widths }
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
    column_widths: Rc<Vec<Units>>
}

impl Table {
    pub fn new<'a, Headers, Body>(cx: &mut Context, headers: Headers, mut body: Body) -> Handle<Self>
    where
        Headers: IntoIterator<Item=&'a str>,
        Body: 'static + FnMut(&mut Context, usize) -> bool
    {
        let headers = headers.into_iter().map(|i| i.to_owned()).collect::<Vec<String>>();
        let widths = Rc::new(vec![Units::Percentage(100.0 / headers.len() as f32); headers.len()]);
        let even = Units::Percentage(100.0 / headers.len() as f32);
        
        Self { 
            headers: headers.clone(),
            column_widths: Rc::clone(&widths)
        }
            .build(cx, move |cx| {
                let headers = headers;
                let widths = widths;
                
                Row::new(cx, |cx| {
                    for (a, i) in headers.iter().enumerate() {
                        HStack::new(cx, |cx| {
                            Label::new(cx, ICON_CHEVRON_DOWN);
                            Label::new(cx, i);
                        })
                            .class("table-header-column");
                    }
                }, Rc::clone(&widths))
                    .class("table-header");
                
                ScrollView::new(cx, 0.0, 0.0, false, true, move |cx| {
                    let mut index: usize = 0;
                    loop {
                        let mut should_break = false;
                        Row::new(cx, |cx| {
                            should_break = !body(cx, index);
                        }, Rc::clone(&widths));
                        
                        index += 1;
                        
                        if should_break {
                            break;
                        }
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
