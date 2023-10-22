from mylib.lib import compute_average
import panda as pd

def main():
  df=pd.read.csv("Auto.csv")
  compute_average(df)

if __name__ == "__main__":
    main()
