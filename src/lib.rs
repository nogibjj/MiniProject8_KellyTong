use polars::prelude::*;
use std::collections::HashMap;
use std::result::Result as StdResult;

pub fn compute_average(df: &DataFrame) -> StdResult<HashMap<String, f64>, polars::error::PolarsError> {
    let average_mpg = df
        .column("mpg")
        .map_err(|_| polars::error::PolarsError::NotFound("mpg column not found".into()))?
        .mean()
        .ok_or_else(|| polars::error::PolarsError::NotFound("Failed to compute mean for mpg".into()))?;

    let average_weight = df
        .column("weight")
        .map_err(|_| polars::error::PolarsError::NotFound("weight column not found".into()))?
        .mean()
        .ok_or_else(|| polars::error::PolarsError::NotFound("Failed to compute mean for weight".into()))?;
    
    let mut averages = HashMap::new();
    averages.insert("mpg average".to_string(), average_mpg);
    averages.insert("weight average".to_string(), average_weight);

    Ok(averages)
}
