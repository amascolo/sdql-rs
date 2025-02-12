use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use itertools::iproduct;
use sdql::q3::parallel::q3_query_rayon;
use sdql::q3::read::read_q3;
use sdql::q3::sequential::q3_query;

const SCALE_FACTORS: &[&str] = &["0.01", "1"];
const PARALLELISM: [bool; 2] = [false, true];

fn benchmark_q3(c: &mut Criterion) {
    for (sf, parallel) in iproduct!(SCALE_FACTORS, PARALLELISM) {
        let path = |table| format!("datasets/tpch_datasets/SF_{sf}/{table}.tbl");
        let data = read_q3(&path("customer"), &path("orders"), &path("lineitem")).unwrap();
        let query = if parallel { q3_query_rayon } else { q3_query };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("q3", format!("{variant}_SF{sf}"));
        c.bench_with_input(id, &data, |b, (customer, orders, lineitem)| {
            b.iter(|| black_box(query(customer, orders, lineitem)))
        });
    }
}

criterion_group!(benches, benchmark_q3);
criterion_main!(benches);
