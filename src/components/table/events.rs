use vizia::prelude::*;
use crate::components::table::*;

pub enum SortDirection {
    Ascending,
    Descending
}

pub enum TableEvents {
    AlterSource { source: &'static [TableRowBuilder] },
    ResizeColumn { column: usize, size: Units },
    SetSortPreference { column: usize, sort_direction: SortDirection }
}

impl View for Table {
    fn element(&self) -> Option<&'static str> {
        Some("table")
    } 
    
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            None => {},
            Some(TableEvents::AlterSource { source }) => {},
            Some(TableEvents::ResizeColumn { column, size }) => {},
            Some(TableEvents::SetSortPreference { column, sort_direction }) => {},
        });
    }
}