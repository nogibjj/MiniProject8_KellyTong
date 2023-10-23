import pandas as pd
from main import compute_average


def test_compute_average():
  df = pd.read_csv("Auto.csv")
  avg = compute_average(df)
  assert "overall average" in avg, "overall average not computed"
  assert "column average" in avg, "column average not computed"
  assert "row average" in avg, "row average not computed"

if __name__ == "__main__":
    test_compute_average()
