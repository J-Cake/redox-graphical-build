use vizia::{prelude::*, ICON_CHEVRON_DOWN};

pub struct Table {
    headers: Vec<String>,
    column_widths: Vec<LengthOrPercentage>
}

impl Table {
    pub fn new<'a, Headers>(cx: &mut Context, headers: Headers) -> Handle<Self>
    where
        Headers: IntoIterator<Item=&'a str>
    {
        let headers: Vec<String> = headers.into_iter().map(|i| i.to_owned()).collect();
        let widths: Vec<LengthOrPercentage> = vec![LengthOrPercentage::Percentage(100.0 / headers.len() as f32); headers.len()];
        
        Self { 
            headers: headers.clone(),
            column_widths: widths.clone()
        }
            .build(cx, move |cx| {
                let headers = headers;
                let widths = widths;
                
                HStack::new(cx, |cx| {
                    for (a, i) in headers.iter().enumerate() {
                        
                        HStack::new(cx, |cx| {
                            Label::new(cx, ICON_CHEVRON_DOWN);
                            Label::new(cx, i);
                        })
                            .width(Percentage(33.3f32))
                            .class("table-header-column");
                    }
                })
                    .class("table-header");
                
                ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                    
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
