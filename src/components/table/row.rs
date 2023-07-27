use vizia::prelude::*;

use super::*;

#[derive(Lens)]
pub struct TableRow<Columns: 'static + Lens<Target=Vec<TableColumnInfo>>> {
    columns: Columns
}

impl<Columns: 'static + Lens<Target=Vec<TableColumnInfo>>> View for TableRow<Columns> {
    fn element(&self) -> Option<&'static str> {
        Some("table-row")
    }
}

impl<Columns: 'static + Lens<Target=Vec<TableColumnInfo>>> TableRow<Columns> {
    pub fn new(cx: &mut Context, columns: Columns) -> Handle<Self> {
        View::build(Self { columns }, cx, |cx| {})
    }
}

pub struct TableRowBuilder {}

impl TableRowBuilder {
    pub fn new() -> Self { Self {} }
    
    pub fn add_column<Body: FnMut(&mut Context)>(self, column: &str, body: Body) -> Self {
        self
    }
    
    pub(crate) fn into_row<Columns: 'static + Lens<Target=Vec<TableColumnInfo>>>(self, cx: &mut Context, columns: Columns) -> Handle<TableRow<Columns>> {
        TableRow::new(cx, columns)
    }
}