extern crate polars;

use polars::prelude::*;
use std::error::Error;

mod lib;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "Auto.csv";
    let df = CsvReader::from_path(file_path)?.has_header(true).finish()?;

    if !df.is_empty() {
        println!("DataFrame is not empty.");

        let avg = lib::compute_average(&df)?;
        match (avg.get("mpg average"), avg.get("weight average")) {
            (Some(mpg_avg), Some(weight_avg)) => {
                println!("mpg average: {:.2}", mpg_avg);
                println!("weight average: {:.2}", weight_avg);
            },
            _ => println!("Failed to compute some averages.")
        }
    } else {
        println!("DataFrame is empty.");
    }

    Ok(())
}




