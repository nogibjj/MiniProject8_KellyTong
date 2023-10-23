extern crate polars;

use polars::prelude::*;
use std::error::Error;

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let file_path = "Auto.csv";
    let df = CsvReader::from_path(file_path)?.has_header(true).finish()?;

    if !df.is_empty() {
        println!("DataFrame is not empty.");

        let avg = mini_project8_kelly_rust::compute_average(&df)?;
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
After making these changes, try running cargo clippy --quiet again and see if the warnings and errors have been resolved.





