import sys

import matplotlib
import pandas as pd
import seaborn as sns

if __name__ == "__main__":
    major, minor = sys.version_info[:2]
    assert (major, minor) == (3, 13)

    assert pd.__version__.startswith("2.2")
    assert matplotlib.__version__.startswith("3.10")
    assert sns.__version__.startswith("0.13")

    print("Hello, plotters.")
