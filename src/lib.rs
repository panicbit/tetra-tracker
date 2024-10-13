#![expect(warnings)]

pub mod cli;
pub mod pack;
pub mod ui;
mod util;

const BOM: &str = "\u{FEFF}";
