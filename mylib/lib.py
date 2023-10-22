def compute_average(df): 
  overall_avg = df.mean().mean()
  column_avg = df.mean()
  row_avg = df.mean(axis=5)
  return {
          'overall average': overall_avg, 
          'column average': column_avg, 
          'row average': row_avg
          }
