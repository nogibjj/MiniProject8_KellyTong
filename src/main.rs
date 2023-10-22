// main.rs
extern crate polars;

use polars::prelude::*;
use std::error::Error;

mod mylib; // import the mylib module

fn main() -> Result<(), Box<dyn Error>> {
    let df = CsvReader::from_path("Auto.csv")?.infer_schema(None).has_header(true).finish()?;
    mylib::compute_average(df)?;

    Ok(())
}
