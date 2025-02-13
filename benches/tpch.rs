use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use sdql::q3::parallel::q3_query_rayon;
use sdql::q3::read::read_q3;
use sdql::q3::sequential::q3_query;

fn benchmark_q3(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let data = read_q3(&path("customer"), &path("orders"), &path("lineitem")).unwrap();
    for parallel in [false, true] {
        let query = if parallel { q3_query_rayon } else { q3_query };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("q3", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (customer, orders, lineitem)| {
            b.iter(|| black_box(query(customer, orders, lineitem)))
        });
    }
}

criterion_group!(benches, benchmark_q3);
criterion_main!(benches);
