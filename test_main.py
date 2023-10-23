import pandas as pd
from main import compute_average


def test_compute_average():
    df = pd.read_csv("Auto.csv")
    avg = compute_average(df)
    assert "weight average" in avg, "weight average not computed"
    assert "mpg average" in avg, "mpg average not computed"


if __name__ == "__main__":
    test_compute_average()
