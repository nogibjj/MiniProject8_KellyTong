import pandas as pd

def compute_average(data): 
  df = pd.DataFrame(data)
  overall_avg = df.mean().mean()
  print(f"Overall Average: {overall_avg}")
  column_avg = df.mean()
  print(column_avg)
  row_avg = df.mean(axis=5)
  print(row_avg)
  return overall_avg, column_avg, row_avg
