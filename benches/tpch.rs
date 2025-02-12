use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use itertools::iproduct;
use sdql::q3::parallel::q3_query_rayon;
use sdql::q3::read::read_q3;
use sdql::q3::read_arrow::read_q3_arrow;
use sdql::q3::read_polars::read_q3_polars;
use sdql::q3::sequential::q3_query;

const CRITERION_MIN_SAMPLE_SIZE: usize = 10;
const SCALE_FACTORS: &[&str] = &["0.01", "1"];
const PARALLELISM: [bool; 2] = [false, true];

fn benchmark_read_q3(_c: &mut Criterion) {
    let mut c = Criterion::default().sample_size(CRITERION_MIN_SAMPLE_SIZE);
    const SF: &str = "1";
    for variant in ["csv", "polars", "arrow"] {
        let path = |table| format!("datasets/tpch_datasets/SF_{SF}/{table}.tbl");
        let read = match variant {
            "csv" => read_q3,
            "polars" => read_q3_polars,
            "arrow" => read_q3_arrow,
            _ => unreachable!(),
        };
        let id = format!("q3_SF{SF}_{variant}");
        c.bench_function(&id, |b| {
            b.iter(|| {
                black_box(read(&path("customer"), &path("orders"), &path("lineitem")).unwrap())
            })
        });
    }
}

fn benchmark_query_q3(c: &mut Criterion) {
    for (sf, parallel) in iproduct!(SCALE_FACTORS, PARALLELISM) {
        let path = |table| format!("datasets/tpch_datasets/SF_{sf}/{table}.tbl");
        let data = read_q3(&path("customer"), &path("orders"), &path("lineitem")).unwrap();
        let query = if parallel { q3_query_rayon } else { q3_query };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("q3", format!("SF{sf}_{variant}"));
        c.bench_with_input(id, &data, |b, (customer, orders, lineitem)| {
            b.iter(|| black_box(query(customer, orders, lineitem)))
        });
    }
}

criterion_group!(benches, benchmark_read_q3, benchmark_query_q3);
criterion_main!(benches);
