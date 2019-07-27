//! Generator for chars(1) data files.

extern crate getopts;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate quick_error;
extern crate fst;
extern crate regex;
extern crate unicode_names2;

use std::error::Error;
use std::path::Path;

mod ascii;
mod fst_generator;
mod unicode;

/// Runs the code generator and writes files.
pub fn generate_files(data_dir: &Path, src_dir: &Path) -> Result<(), Box<dyn Error>> {
    let mut sorted_names = fst_generator::Names::new();

    ascii::write_ascii_name_data(
        &data_dir.join("ascii/nametable"),
        &src_dir.join("ascii"),
        &mut sorted_names,
    );
    unicode::read_names(&mut sorted_names, &data_dir.join("unicode/NameAliases.txt"))?;
    unicode::read_names(&mut sorted_names, &data_dir.join("unicode/UnicodeData.txt"))?;
    unicode::write_name_data(&sorted_names, &src_dir.join("unicode/"))?;
    Ok(())
}