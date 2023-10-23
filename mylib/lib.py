def compute_average(df):
  row_avg = df.mean(axis=5)
  row_avg2 = df.mean(axis=1)
  return {
          'weight average': row_avg,
          'mpg average': row_avg2
          }
