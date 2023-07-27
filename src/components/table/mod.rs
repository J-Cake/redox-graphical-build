use std::collections::VecDeque;

use vizia::prelude::*;

mod row;
mod cell;
mod events;
mod header;

pub use row::*;
pub use cell::*;

use self::header::TableHeader;

#[derive(Clone, Data)]
pub struct TableColumnInfo {
    width: Units,
    heading: String,
    sort_index: usize
}

impl Default for TableColumnInfo {
    fn default() -> Self {
        Self {
            width: Units::Stretch(1.0),
            heading: "".to_owned(),
            sort_index: 0
        }
    }
}

#[derive(Lens)]
pub struct Table {
    columns: Vec<TableColumnInfo>,
    body: Vec<TableRowBuilder>,
    sorting: VecDeque<usize>
}

impl Table {
    pub fn new<Headings, Body>(cx: &mut Context, headings: Headings, body: Body) -> Handle<Self>
    where 
        Body: 'static + IntoIterator<Item=TableRowBuilder>,
        Headings: 'static + IntoIterator<Item=&'static str> {
        View::build(Self { 
            body: vec![],
            sorting: VecDeque::new(),
            columns: headings.into_iter()
                .enumerate()
                .map(|(a, i)| TableColumnInfo {
                    heading: i.to_owned(),
                    sort_index: a,
                    width: Units::Stretch(1.0)
                })
                .collect()
        }, cx, |cx| {
            Binding::new(cx, Table::columns, |cx, item| {
                let cols = item.get(cx);
                TableHeader::new(cx, cols);
                
                ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                    Binding::new(cx, Table::body, |cx, body| {
                        for i in body.get(cx) {
                            i
                        }
                    })
                })
            });
        })
    }
}