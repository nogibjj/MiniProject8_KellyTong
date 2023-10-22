// main.rs
extern crate polars;

use polars::prelude::*;
use std::error::Error;

mod lib; // import the lib module

fn main() -> Result<(), Box<dyn Error>> {
    let df = CsvReader::from_path("Auto.csv")?.infer_schema(None).has_header(true).finish()?;
    mylib::compute_average(df)?;

    Ok(())
}
