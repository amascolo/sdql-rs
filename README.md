# Semiring Dictionary Query Language (SDQL) in Rust 🦀

Run with `cargo bench` and print results with `./print_benches.sh`

<!--
### Usage

```
cargo [compile|interpret] <SDQL_PATH>
```

e.g.

```
cargo compile progs/tpch/6.sdql
```
-->

Covers TPC-H queries from:

* A. Shaikhha, M. Huot, S. Hashemian, A. Kaboli, **A. Mascolo**, M. Nikolic, J. Smith, D. Olteanu.\
  *A Semi-ring Dictionary Query Language for Data Science*\
  www.research.ed.ac.uk/en/publications/a-semi-ring-dictionary-query-language-for-data-science

JOB and LSQB queries are in:

* A. Kaboli, **A. Mascolo**, A. Shaikhha.\
  *A Unified Architecture for Efficient Binary and Worst-Case Optimal Join Processing*\
  https://arxiv.org/abs/2505.19918 ⋅ Scala and C++ https://github.com/edin-dal/sdql/tree/sorted-dict
