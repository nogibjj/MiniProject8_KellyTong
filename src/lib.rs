use polars::prelude::*;
use std::collections::HashMap;
use std::result::Result as StdResult;

pub fn compute_average(df: &DataFrame) -> StdResult<HashMap<String, f64>, polars::error::PolarsError> {
    let average_mpg = df.column("mpg")?
        .mean()?
        .get(0)
        .and_then(|v| v.get().ok_or_else(|| polars::error::PolarsError::ComputeError("Mean computation failed".into())))
        .map(|v| v.to_f64());

    let average_weight = df.column("weight")?
        .mean()?
        .get(0)
        .and_then(|v| v.get().ok_or_else(|| polars::error::PolarsError::ComputeError("Mean computation failed".into())))
        .map(|v| v.to_f64());

    match (average_mpg, average_weight) {
        (Some(mpg), Some(weight)) => {
            let mut map = HashMap::new();
            map.insert("weight average".to_string(), weight);
            map.insert("mpg average".to_string(), mpg);
            Ok(map)
        }
        _ => Err(polars::error::PolarsError::ComputeError("Failed to compute averages".into()))
    }
}
