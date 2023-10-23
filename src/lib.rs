use polars::prelude::*;

fn compute_average(df: DataFrame) -> Result<HashMap<&'static str, Series>, PolarsError> {
    let overall_avg = df.mean()?.mean();
    let column_avg = df.mean()?;
    let row_avg = df.mean_axis(Axis::Horizontal)?;

    let mut averages = HashMap::new();
    averages.insert("overall average", Series::new("", &[overall_avg]));
    averages.insert("column average", column_avg);
    averages.insert("row average", row_avg);

    Ok(averages)
}
