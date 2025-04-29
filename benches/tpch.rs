use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use sdql::tpch::q1::{q1_query, q1_query_rayon};
use sdql::tpch::q10::{q10_query, q10_query_rayon};
use sdql::tpch::q11::{q11_query, q11_query_rayon};
use sdql::tpch::q12::{q12_query, q12_query_rayon};
use sdql::tpch::q13::{q13_query, q13_query_rayon};
use sdql::tpch::q14::{q14_query, q14_query_rayon};
use sdql::tpch::q15::{q15_query, q15_query_rayon};
use sdql::tpch::q16::{q16_query, q16_query_rayon};
use sdql::tpch::q17::{q17_query, q17_query_rayon};
use sdql::tpch::q18::{q18_query, q18_query_rayon};
use sdql::tpch::q19::{q19_query, q19_query_rayon};
use sdql::tpch::q2::{q2_query, q2_query_rayon};
use sdql::tpch::q20::{q20_query, q20_query_rayon};
use sdql::tpch::q21::{q21_query, q21_query_rayon};
use sdql::tpch::q22::{q22_query, q22_query_rayon};
use sdql::tpch::q3::{q3_query, q3_query_rayon};
use sdql::tpch::q4::{q4_query, q4_query_rayon};
use sdql::tpch::q5::{q5_query, q5_query_rayon};
use sdql::tpch::q6::{q6_query, q6_query_rayon};
use sdql::tpch::q7::{q7_query, q7_query_rayon};
use sdql::tpch::q8::{q8_query, q8_query_rayon};
use sdql::tpch::q9::{q9_query, q9_query_rayon};
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

fn benchmark_q9(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let part = read_part()(&path("part")).unwrap();
    let supplier = read_supplier()(&path("lineitem")).unwrap();
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let partsupp = read_partsupp()(&path("partsupp")).unwrap();
    let orders = read_orders()(&path("orders")).unwrap();
    let nation = read_nation()(&path("nation")).unwrap();
    let data = (part, supplier, lineitems, partsupp, orders, nation);
    for parallel in [false, true] {
        let query = if parallel { q9_query_rayon } else { q9_query };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("q9", format!("SF1_{variant}"));
        c.bench_with_input(
            id,
            &data,
            |b, (part, supplier, lineitems, partsupp, orders, nation)| {
                b.iter(|| black_box(query(part, supplier, lineitems, partsupp, orders, nation)))
            },
        );
    }
}

fn benchmark_q10(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let customers = read_customers()(&path("customer")).unwrap();
    let orders = read_orders()(&path("orders")).unwrap();
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let nation = read_nation()(&path("nation")).unwrap();
    let data = (customers, orders, lineitems, nation);
    for parallel in [false, true] {
        let query = if parallel { q10_query_rayon } else { q10_query };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("q10", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (customers, orders, lineitems, nation)| {
            b.iter(|| black_box(query(customers, orders, lineitems, nation)))
        });
    }
}

fn benchmark_q11(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let supplier = read_supplier()(&path("supplier")).unwrap();
    let partsupp = read_partsupp()(&path("partsupp")).unwrap();
    let nation = read_nation()(&path("nation")).unwrap();
    let data = (supplier, partsupp, nation);
    for parallel in [false, true] {
        let query = if parallel { q11_query_rayon } else { q11_query };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("q11", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (supplier, partsupp, nation)| {
            b.iter(|| black_box(query(supplier, partsupp, nation)))
        });
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

fn benchmark_q13(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let orders = read_orders()(&path("orders")).unwrap();
    let customer = read_customers()(&path("customer")).unwrap();
    let data = (orders, customer);
    for parallel in [false, true] {
        let query = if parallel { q13_query_rayon } else { q13_query };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("q13", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (orders, customer)| {
            b.iter(|| black_box(query(orders, customer)))
        });
    }
}

fn benchmark_q14(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let part = read_part()(&path("part")).unwrap();
    let data = (lineitems, part);
    for parallel in [false, true] {
        let query = if parallel { q14_query_rayon } else { q14_query };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("q14", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (lineitems, part)| {
            b.iter(|| black_box(query(lineitems, part)))
        });
    }
}

fn benchmark_q15(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let supplier = read_supplier()(&path("supplier")).unwrap();
    let data = (lineitems, supplier);
    for parallel in [false, true] {
        let query = if parallel { q15_query_rayon } else { q15_query };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("q15", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (lineitems, supplier)| {
            b.iter(|| black_box(query(lineitems, supplier)))
        });
    }
}

fn benchmark_q16(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let partsupp = read_partsupp()(&path("partsupp")).unwrap();
    let part = read_part()(&path("part")).unwrap();
    let supplier = read_supplier()(&path("supplier")).unwrap();
    let data = (partsupp, part, supplier);
    for parallel in [false, true] {
        let query = if parallel { q16_query_rayon } else { q16_query };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("q16", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (partsupp, part, supplier)| {
            b.iter(|| black_box(query(partsupp, part, supplier)))
        });
    }
}

fn benchmark_q17(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let part = read_part()(&path("part")).unwrap();
    let data = (lineitems, part);
    for parallel in [false, true] {
        let query = if parallel { q17_query_rayon } else { q17_query };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("q17", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (lineitems, part)| {
            b.iter(|| black_box(query(lineitems, part)))
        });
    }
}

fn benchmark_q18(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let customers = read_customers()(&path("customer")).unwrap();
    let orders = read_orders()(&path("orders")).unwrap();
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let data = (customers, orders, lineitems);
    for parallel in [false, true] {
        let query = if parallel { q18_query_rayon } else { q18_query };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("q18", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (customers, orders, lineitems)| {
            b.iter(|| black_box(query(customers, orders, lineitems)))
        });
    }
}

fn benchmark_q19(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let part = read_part()(&path("part")).unwrap();
    let data = (lineitems, part);
    for parallel in [false, true] {
        let query = if parallel { q19_query_rayon } else { q19_query };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("q19", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (lineitems, part)| {
            b.iter(|| black_box(query(lineitems, part)))
        });
    }
}

fn benchmark_q20(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let supplier = read_supplier()(&path("supplier")).unwrap();
    let nation = read_nation()(&path("nation")).unwrap();
    let part = read_part()(&path("part")).unwrap();
    let partsupp = read_partsupp()(&path("partsupp")).unwrap();
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let data = (supplier, nation, part, partsupp, lineitems);
    for parallel in [false, true] {
        let query = if parallel { q20_query_rayon } else { q20_query };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("q20", format!("SF1_{variant}"));
        c.bench_with_input(
            id,
            &data,
            |b, (supplier, nation, part, partsupp, lineitems)| {
                b.iter(|| black_box(query(supplier, nation, part, partsupp, lineitems)))
            },
        );
    }
}

fn benchmark_q21(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let supplier = read_supplier()(&path("lineitem")).unwrap();
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let orders = read_orders()(&path("orders")).unwrap();
    let nation = read_nation()(&path("nation")).unwrap();
    let data = (supplier, lineitems, orders, nation);
    for parallel in [false, true] {
        let query = if parallel { q21_query_rayon } else { q21_query };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("q21", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (supplier, lineitems, orders, nation)| {
            b.iter(|| black_box(query(supplier, lineitems, orders, nation)))
        });
    }
}

fn benchmark_q22(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let customer = read_customers()(&path("customer")).unwrap();
    let orders = read_orders()(&path("orders")).unwrap();
    let data = (customer, orders);
    for parallel in [false, true] {
        let query = if parallel { q22_query_rayon } else { q22_query };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("q22", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (customer, orders)| {
            b.iter(|| black_box(query(customer, orders)))
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
    benchmark_q9,
    benchmark_q10,
    benchmark_q11,
    benchmark_q12,
    benchmark_q13,
    benchmark_q14,
    benchmark_q15,
    benchmark_q16,
    benchmark_q17,
    benchmark_q18,
    benchmark_q19,
    // TODO
    benchmark_q20,
    benchmark_q21,
    benchmark_q22,
);
criterion_main!(benches);
