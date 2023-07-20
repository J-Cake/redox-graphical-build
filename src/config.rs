use vizia::prelude::*;
use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
  
}

impl Data for BuildConfig {
    fn same(&self, other: &Self) -> bool {
        true
    }
}