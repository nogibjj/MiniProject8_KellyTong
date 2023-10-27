use polars::prelude::*;
use std::collections::HashMap;
use std::result::Result as StdResult;

pub fn compute_average(
    df: &DataFrame,
) -> StdResult<HashMap<String, f64>, String> {
    let average_mpg = df
        .column("mpg")
        .map_err(|_| "mpg column not found".to_string())?
        .mean()
        .ok_or_else(|| "Failed to compute mean for mpg".to_string())?;

    let average_weight = df
        .column("weight")
        .map_err(|_| "weight column not found".to_string())?
        .mean()
        .ok_or_else(|| "Failed to compute mean for weight".to_string())?;

    let mut averages = HashMap::new();
    averages.insert("mpg average".to_string(), average_mpg);
    averages.insert("weight average".to_string(), average_weight);

    Ok(averages)
}
