import sys

import matplotlib
import pandas as pd
import seaborn as sns

if __name__ == "__main__":
    df = pd.read_excel("../benchmarks.xlsx")

    print(df.head())
