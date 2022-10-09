use eyre::Result;

use crate::parser::parser::ComposeItem;
use std::{ffi::OsStr, path::Path};

#[derive(PartialEq)]
pub struct System {}

impl System {
    pub fn init() -> Self {
        System {}
    }

    pub fn cd<'a>(&'a self, item: &'a ComposeItem) -> Result<&str> {
        // Get path from a compose item
        let path = Path::new(OsStr::new(&item.compose_files[0]))
            .parent()
            .unwrap();

        let path_str = path.to_str().unwrap();

        Ok(&path_str)
    }
}
