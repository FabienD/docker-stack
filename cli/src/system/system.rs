use eyre::Result;

use crate::parser::parser::ComposeItem;
use std::{ffi::OsStr, path::Path};

pub struct System {}

impl System {
    pub fn init() -> Self {
        System {}
    }

    pub fn cd(&self, item: &ComposeItem) -> Result<()> {
        // Get path from a compose item
        let path = Path::new(OsStr::new(&item.compose_files[0]))
            .parent()
            .unwrap();

        println!("{}", path.to_str().unwrap());

        Ok(())
    }
}
