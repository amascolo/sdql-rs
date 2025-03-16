use sdql::cli::run_tpch;

#[test]
fn tpch() {
    run_tpch(1, "0.01").unwrap();
    run_tpch(3, "0.01").unwrap();
    run_tpch(5, "0.01").unwrap();
    run_tpch(6, "0.01").unwrap();
    run_tpch(9, "0.01").unwrap();
    run_tpch(18, "0.01").unwrap();
}
