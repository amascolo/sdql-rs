use sdql::cli::run_tpch;

// FIXME make it fail when it prints output!
#[test]
fn tpch() {
    run_tpch(1, "0.01");
    run_tpch(3, "0.01");
    run_tpch(5, "0.01");
    run_tpch(6, "0.01");
    run_tpch(9, "0.01");
    run_tpch(18, "0.01");
}
