extern crate polars;

use polars::prelude::*;
use std::error::Error;
use std::result::Result as StdResult;

mod lib;

fn main() -> StdResult<(), Box<dyn Error>> {
    let file_path = "Auto.csv";
    
    let df = CsvReader::from_path(file_path)?.infer_schema(None).has_header(true).finish()?;

    if !df.is_empty() {
        println!("DataFrame is not empty.");

        let avg = lib::compute_average(&df)?;
        match (avg.get("overall average"), avg.get("column average"), avg.get("row average")) {
            (Some(overall_avg), Some(column_avg), Some(row_avg)) => {
                println!("overall average: {:.2}", overall_avg);
                println!("column average: {:.2}", column_avg);
                println!("row average: {:.2}", row_avg);
            },
            _ => println!("Failed to compute some averages.")
        }
    } else {
        println!("DataFrame is empty.");
    }

    Ok(())
}
