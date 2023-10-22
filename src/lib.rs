fn compute_average(data: HashMap<&str, Vec<f64>>) -> (f64, HashMap<&str, f64>, Vec<f64>) {
    let mut df = DataFrame::from(data);
    
    // Overall Average
    let overall_avg = df.mean().mean();
    println!("Overall Average: {}", overall_avg);

    // Column Average
    let mut column_avg = HashMap::new();
    for column in df.columns() {
        column_avg.insert(column, df.column(column).mean());
    }
    for (column, avg) in &column_avg {
        println!("{}: {}", column, avg);
    }

    // Row Average
    let row_avg = df.mean_axis(Axis::Horizontal);
    for avg in &row_avg {
        println!("{}", avg);
    }

    (overall_avg, column_avg, row_avg)
}
