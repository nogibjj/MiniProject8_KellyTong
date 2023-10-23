extern crate ndarray;
use ndarray::{Array2, Axis};
use std::collections::HashMap;

pub fn compute_average(df: &Array2<f64>) -> Result<HashMap<String, f64>, &'static str> {
    // Select only numeric columns - since we are using ndarray and assuming f64, this step is skipped

    let overall_avg = df.mean().ok_or("Failed to compute overall average")?;
    
    // Compute mean of each column
    let column_avg = df.mean_axis(Axis(0)).ok_or("Failed to compute column average")?;
    
    // Compute mean of each row
    let row_avg = df.mean_axis(Axis(1)).ok_or("Failed to compute row average")?;
    
    // We'll use a HashMap to return the results similar to a dictionary in Python
    let mut result = HashMap::new();
    result.insert("overall average".to_string(), overall_avg);
    result.insert("column average".to_string(), column_avg.mean().ok_or("Failed to compute column average mean")?);
    result.insert("row average".to_string(), row_avg.mean().ok_or("Failed to compute row average mean")?);
    
    Ok(result)
}
