import os
import warnings
from enum import Enum
from pathlib import Path
from typing import Final

import matplotlib.pyplot as plt
import pandas as pd
import seaborn as sns

warnings.filterwarnings(
    "ignore", category=FutureWarning, message=".*length-1 list-like.*get_group.*"
)

DIR: Final[str] = os.path.dirname(os.path.realpath(__file__))


class Engine(Enum):
    SDQL = "sdql"
    SDQL_RS = "sdql-rs"
    DUCK_DB = "DuckDB"
    SDQL_RS_PARALLEL = "sdql-rs (parallel)"
    DUCK_DB_PARALLEL = "DuckDB (parallel)"


def plot(df: pd.DataFrame, us: Engine, cmp: Engine) -> None:
    us = us.value
    cmp = cmp.value

    df = df.copy()
    df = df[[us, cmp]]
    df = df.head(22)
    df["Query"] = "Q" + (df.index + 1).astype(str)
    df_melt = df.melt(
        id_vars="Query",
        value_vars=[us, cmp],
        var_name="Engine",
        value_name="Run Time (ms)",
    )

    sns.set_theme(style="whitegrid", context="talk")
    fig, ax = plt.subplots(figsize=(14, 6))
    sns.barplot(
        data=df_melt,
        x="Query",
        y="Run Time (ms)",
        hue="Engine",
        palette=sns.color_palette("tab10", 2),
        edgecolor=".1",
        ax=ax,
    )

    ax.set_yscale("log")
    # get rid of silly 10⁰ label
    ticks = ax.get_yticks()
    ticks = ticks[2:]
    ax.set_yticks(ticks)

    ax.set_xlabel("")
    ax.set_ylabel("Run Time (ms)")
    ax.set_ylim(1, df_melt["Run Time (ms)"].max() * 1.1)
    ax.legend(
        title="",
        loc="upper left",
        bbox_to_anchor=(0.05, 1.00),
        bbox_transform=ax.transAxes,
    )
    plt.xticks(rotation=45)
    plt.tight_layout()

    path = Path(DIR) / f"{us} vs {cmp}.pdf"
    plt.savefig(path, bbox_inches="tight")


if __name__ == "__main__":
    df = pd.read_excel("../benchmarks.xlsx")
    plot(df, us=Engine.SDQL, cmp=Engine.DUCK_DB)
    plot(df, us=Engine.SDQL_RS, cmp=Engine.DUCK_DB)
    plot(df, us=Engine.SDQL_RS_PARALLEL, cmp=Engine.DUCK_DB_PARALLEL)
