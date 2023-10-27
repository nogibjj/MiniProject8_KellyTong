extern crate polars;
extern crate sys_info;
use polars::prelude::*;
use polars::prelude::DataFrame;
use std::error::Error;
use std::result::Result as StdResult;
use std::process::Command;
use std::time::Instant;
use csv::Reader as CsvReader;

mod lib;

fn main() -> StdResult<(), Box<dyn Error>> {
    let start_time = Instant::now();
    
    let file_path = "Auto.csv";

    let df = CsvReader::from_path(file_path)?
        .infer_schema(None)
        .has_header(true)
        .finish()?;

    if !df.is_empty() {
        println!("DataFrame is not empty.");

        let avg = lib::compute_average(&df)?;
        match (
            avg.get("average_mpg"),
            avg.get("average_weight"),
        ) {
            (Some(overall_avg), Some(column_avg), Some(row_avg)) => {
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
                                                          // Memory usage
    let mem_info = sys_info::mem_info().unwrap();
    println!(
        "Memory Usage: {}%",
        (mem_info.total - mem_info.avail) as f32 / mem_info.total as f32 * 100.0
    );
    // CPU calculation
    let output = Command::new("ps")
        .arg("-o")
        .arg("%cpu")
        .arg("-p")
        .arg(format!("{}", std::process::id()))
        .output()
        .expect("Failed to execute ps command");

    // Convert the output to a string
    let usage = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = usage.split('\n').collect();

    // Parse the CPU usage from the output
    if lines.len() >= 2 {
        let usage_str = lines[1].trim();
        let usage_float: StdResult<f32, _> = usage_str.parse();
        match usage_float {
            Ok(usage) => println!("CPU Usage: {:.2}%", usage),
            Err(_) => println!("Failed to parse CPU usage"),
        }
    } else {
        println!("Failed to get CPU usage");
    }
    
    Ok(())
}
