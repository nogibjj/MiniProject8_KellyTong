extern crate polars;

use polars::prelude::*;
use std::collections::HashMap;
use std::error::Error;

mod lib;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "Auto.csv";
    let df = DataFrame::read_csv(file_path, CsvReader::default())?;

    if !df.is_empty() {
        println!("DataFrame is not empty.");

        let avg = mylib::compute_average(&df)?;
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
