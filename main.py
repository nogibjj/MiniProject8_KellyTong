from mylib.lib import compute_average
import pandas as pd
import time
import psutil

# Record the start time
start_time = time.time()


def main():
    df = pd.read_csv("Auto.csv")
    if not df.empty:
        print("DataFrame is not empty.")
        avg = compute_average(df)
        print(f"average mpg: {avg['mpg average']:.2f}")
        print(f"average weight: {avg['weight average']:.2f}")
    else:
        print("DataFrame is empty.")

    end_time = time.time()

    # Calculate the elapsed time
    elapsed_time = end_time - start_time
    cpu_percent = psutil.cpu_percent()
    memory_info = psutil.virtual_memory()

    print(f"Elapsed time: {elapsed_time:.4f} seconds")
    print(f"CPU Usage: {cpu_percent}%")
    print(f"Memory Usage: {memory_info.percent}%")


if __name__ == "__main__":
    main()
