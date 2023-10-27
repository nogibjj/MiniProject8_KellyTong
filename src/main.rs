extern crate polars;
extern crate sys_info;

use polars::prelude::*;
use std::error::Error;
use polars::prelude::CsvReader;
use std::time::Instant;

mod lib;

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();

    let file_path = "Auto.csv";

    // Use polars CsvReader instead
    let df = CsvReader::from_path(file_path)?
        .infer_schema(None)
        .has_header(true)
        .finish()?;

    if !df.is_empty() {
        println!("DataFrame is not empty.");

        let avg = lib::compute_average(&df)?;
        match (avg.get("mpg average"), avg.get("weight average")) {
            (Some(average_mpg), Some(average_weight)) => {
                println!("average_mpg: {:.2}", average_mpg);
                println!("average_weight: {:.2}", average_weight);
            }
            _ => println!("Failed to compute some averages."),
        }
    } else {
        println!("DataFrame is empty.");
    }

    let end_time = Instant::now();
    // Calculate the elapsed time and resource usage
    let elapsed_time = end_time.duration_since(start_time);
    println!("Total execution time: {:?}", elapsed_time); // Print the elapsed time

    // ... rest of the code remains unchanged ...
    
    Ok(())
}
