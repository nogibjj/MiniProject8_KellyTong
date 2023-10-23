def compute_average(df):
  average_mpg = df["mpg"].mean()
  average_weight = df["weight"].mean()
  return {
          'weight average': average_weight,
          'mpg average': average_mpg
          }
