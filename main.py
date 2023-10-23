from mylib.lib import compute_average
import panda as pd

def main():
  df=pd.read.csv("Auto.csv")
  if not df.empty:
    print("DataFrame is not empty.")
    avg = compute_average(df)
    print(f"overall average: {avg['overall_avg']:.2f}")
    print(f"column average: {avg['column_avg']:.2f}")
    print(f"row average: {avg['row_avg']:.2f}")
  else:
    print("DataFrame is empty.")


if __name__ == "__main__":
    main()
