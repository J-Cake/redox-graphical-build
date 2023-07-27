use vizia::prelude::*;

use crate::components::table::*;

pub struct TableHeader {
    columns: Vec<TableColumnInfo>
}

impl View for TableHeader {
    fn element(&self) -> Option<&'static str> {
        Some("table-header")
    }
}

impl TableHeader {
    pub(crate) fn new<'a>(cx: &'a mut Context, columns: Vec<TableColumnInfo>) -> Handle<'a, Self> {
        View::build(Self { columns }, cx, |cx| {})
    }
}