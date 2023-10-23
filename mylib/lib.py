import numpy as np

def compute_average(df):
  # Select only numeric columns
  numeric_df = df.select_dtypes(include=[np.number])
  # Compute the mean of the numeric columns
  overall_avg = numeric_df.mean().mean()
  column_avg = df.mean()
  row_avg = df.mean(axis=5)
  return {
          'overall average': overall_avg, 
          'column average': column_avg, 
          'row average': row_avg
          }
