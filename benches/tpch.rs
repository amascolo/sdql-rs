use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use sdql::tpch::q1::{q1_query, q1_query_rayon};
use sdql::tpch::q12::{q12_query, q12_query_rayon};
use sdql::tpch::q2::{q2_query, q2_query_rayon};
use sdql::tpch::q3::{q3_query, q3_query_rayon};
use sdql::tpch::q4::{q4_query, q4_query_rayon};
use sdql::tpch::q5::{q5_query, q5_query_rayon};
use sdql::tpch::q6::{q6_query, q6_query_rayon};
use sdql::tpch::q7::{q7_query, q7_query_rayon};
use sdql::tpch::q8::{q8_query, q8_query_rayon};
use sdql::tpch::read::*;

fn benchmark_q1(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let data = (lineitems,);
    for parallel in [false, true] {
        let query = if parallel { q1_query_rayon } else { q1_query };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("q1", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (lineitem,)| {
            b.iter(|| black_box(query(lineitem)))
        });
    }
}

fn benchmark_q2(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let part = read_part()(&path("part")).unwrap();
    let supplier = read_supplier()(&path("supplier")).unwrap();
    let partsupp = read_partsupp()(&path("partsupp")).unwrap();
    let nation = read_nation()(&path("nation")).unwrap();
    let region = read_region()(&path("region")).unwrap();
    let data = (part, supplier, partsupp, nation, region);
    for parallel in [false, true] {
        let query = if parallel { q2_query_rayon } else { q2_query };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("q2", format!("SF1_{variant}"));
        c.bench_with_input(
            id,
            &data,
            |b, (part, supplier, partsupp, nation, region)| {
                b.iter(|| black_box(query(part, supplier, partsupp, nation, region)))
            },
        );
    }
}

fn benchmark_q3(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let customers = read_customers()(&path("customer")).unwrap();
    let orders = read_orders()(&path("orders")).unwrap();
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let data = (customers, orders, lineitems);
    for parallel in [false, true] {
        let query = if parallel { q3_query_rayon } else { q3_query };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("q3", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (customer, orders, lineitem)| {
            b.iter(|| black_box(query(customer, orders, lineitem)))
        });
    }
}

fn benchmark_q4(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let orders = read_orders()(&path("orders")).unwrap();
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let data = (orders, lineitems);
    for parallel in [false, true] {
        let query = if parallel { q4_query_rayon } else { q4_query };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("q4", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (orders, lineitem)| {
            b.iter(|| black_box(query(orders, lineitem)))
        });
    }
}

fn benchmark_q5(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let customers = read_customers()(&path("customer")).unwrap();
    let orders = read_orders()(&path("orders")).unwrap();
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let supplier = read_supplier()(&path("supplier")).unwrap();
    let nation = read_nation()(&path("nation")).unwrap();
    let region = read_region()(&path("region")).unwrap();
    let data = (customers, orders, lineitems, supplier, nation, region);
    for parallel in [false, true] {
        let query = if parallel { q5_query_rayon } else { q5_query };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("q5", format!("SF1_{variant}"));
        c.bench_with_input(
            id,
            &data,
            |b, (customers, orders, lineitems, supplier, nation, region)| {
                b.iter(|| {
                    black_box(query(
                        customers, orders, lineitems, supplier, nation, region,
                    ))
                })
            },
        );
    }
}

fn benchmark_q6(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let data = (lineitems,);
    for parallel in [false, true] {
        let query = if parallel { q6_query_rayon } else { q6_query };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("q6", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (lineitem,)| {
            b.iter(|| black_box(query(lineitem)))
        });
    }
}

fn benchmark_q7(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let supplier = read_supplier()(&path("lineitem")).unwrap();
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let orders = read_orders()(&path("orders")).unwrap();
    let customers = read_customers()(&path("customer")).unwrap();
    let nation = read_nation()(&path("nation")).unwrap();
    let data = (supplier, lineitems, orders, customers, nation);
    for parallel in [false, true] {
        let query = if parallel { q7_query_rayon } else { q7_query };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("q7", format!("SF1_{variant}"));
        c.bench_with_input(
            id,
            &data,
            |b, (supplier, lineitems, orders, customers, nation)| {
                b.iter(|| black_box(query(supplier, lineitems, orders, customers, nation)))
            },
        );
    }
}

fn benchmark_q8(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let part = read_part()(&path("part")).unwrap();
    let supplier = read_supplier()(&path("lineitem")).unwrap();
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let orders = read_orders()(&path("orders")).unwrap();
    let customers = read_customers()(&path("customer")).unwrap();
    let nation = read_nation()(&path("nation")).unwrap();
    let region = read_region()(&path("region")).unwrap();
    let data = (part, supplier, lineitems, orders, customers, nation, region);
    for parallel in [false, true] {
        let query = if parallel { q8_query_rayon } else { q8_query };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("q8", format!("SF1_{variant}"));
        c.bench_with_input(
            id,
            &data,
            |b, (part, supplier, lineitems, orders, customers, nation, region)| {
                b.iter(|| {
                    black_box(query(
                        part, supplier, lineitems, orders, customers, nation, region,
                    ))
                })
            },
        );
    }
}

fn benchmark_q12(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let orders = read_orders()(&path("orders")).unwrap();
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let data = (orders, lineitems);
    for parallel in [false, true] {
        let query = if parallel { q12_query_rayon } else { q12_query };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("q12", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (orders, lineitem)| {
            b.iter(|| black_box(query(orders, lineitem)))
        });
    }
}

criterion_group!(
    benches,
    benchmark_q1,
    benchmark_q2,
    benchmark_q3, // TODO generated, not manually written
    benchmark_q4,
    benchmark_q5,
    benchmark_q6, // TODO generated, not manually written
    benchmark_q7,
    benchmark_q8,
    benchmark_q12
);
criterion_main!(benches);
