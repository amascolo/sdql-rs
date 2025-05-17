use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;

use sdql::tpch::read::*;
use sdql::tpch::_01::{tpch_01, tpch_01_parallel};
use sdql::tpch::_02::{tpch_02, tpch_02_parallel};
use sdql::tpch::_03::{tpch_03, tpch_03_parallel};
use sdql::tpch::_04::{tpch_04, tpch_04_parallel};
use sdql::tpch::_05::{tpch_05, tpch_05_parallel};
use sdql::tpch::_06::{tpch_06, tpch_06_parallel};
use sdql::tpch::_07::{tpch_07, tpch_07_parallel};
use sdql::tpch::_08::{tpch_08, tpch_08_parallel};
use sdql::tpch::_09::{tpch_09, tpch_09_parallel};
use sdql::tpch::_10::{tpch_10, tpch_10_parallel};
use sdql::tpch::_11::{tpch_11, tpch_11_parallel};
use sdql::tpch::_12::{tpch_12, tpch_12_parallel};
use sdql::tpch::_13::{tpch_13, tpch_13_parallel};
use sdql::tpch::_14::{tpch_14, tpch_14_parallel};
use sdql::tpch::_15::{tpch_15, tpch_15_parallel};
use sdql::tpch::_16::{tpch_16, tpch_16_parallel};
use sdql::tpch::_17::{tpch_17, tpch_17_parallel};
use sdql::tpch::_18::{tpch_18, tpch_18_parallel};
use sdql::tpch::_19::{tpch_19, tpch_19_parallel};
use sdql::tpch::_20::{tpch_20, tpch_20_parallel};
use sdql::tpch::_21::{tpch_21, tpch_21_parallel};
use sdql::tpch::_22::{tpch_22, tpch_22_parallel};

fn benchmark_tpch_01(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let data = (lineitems,);
    for parallel in [false, true] {
        let query = if parallel { tpch_01_parallel } else { tpch_01 };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("tpch_01", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (lineitem,)| {
            b.iter(|| black_box(query(lineitem)))
        });
    }
}

fn benchmark_tpch_02(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let part = read_part()(&path("part")).unwrap();
    let supplier = read_supplier()(&path("supplier")).unwrap();
    let partsupp = read_partsupp()(&path("partsupp")).unwrap();
    let nation = read_nation()(&path("nation")).unwrap();
    let region = read_region()(&path("region")).unwrap();
    let data = (part, supplier, partsupp, nation, region);
    for parallel in [false, true] {
        let query = if parallel { tpch_02_parallel } else { tpch_02 };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("tpch_02", format!("SF1_{variant}"));
        c.bench_with_input(
            id,
            &data,
            |b, (part, supplier, partsupp, nation, region)| {
                b.iter(|| black_box(query(part, supplier, partsupp, nation, region)))
            },
        );
    }
}

fn benchmark_tpch_03(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let customers = read_customers()(&path("customer")).unwrap();
    let orders = read_orders()(&path("orders")).unwrap();
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let data = (customers, orders, lineitems);
    for parallel in [false, true] {
        let query = if parallel { tpch_03_parallel } else { tpch_03 };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("tpch_03", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (customer, orders, lineitem)| {
            b.iter(|| black_box(query(customer, orders, lineitem)))
        });
    }
}

fn benchmark_tpch_04(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let orders = read_orders()(&path("orders")).unwrap();
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let data = (orders, lineitems);
    for parallel in [false, true] {
        let query = if parallel { tpch_04_parallel } else { tpch_04 };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("tpch_04", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (orders, lineitem)| {
            b.iter(|| black_box(query(orders, lineitem)))
        });
    }
}

fn benchmark_tpch_05(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let customers = read_customers()(&path("customer")).unwrap();
    let orders = read_orders()(&path("orders")).unwrap();
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let supplier = read_supplier()(&path("supplier")).unwrap();
    let nation = read_nation()(&path("nation")).unwrap();
    let region = read_region()(&path("region")).unwrap();
    let data = (customers, orders, lineitems, supplier, nation, region);
    for parallel in [false, true] {
        let query = if parallel { tpch_05_parallel } else { tpch_05 };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("tpch_05", format!("SF1_{variant}"));
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

fn benchmark_tpch_06(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let data = (lineitems,);
    for parallel in [false, true] {
        let query = if parallel { tpch_06_parallel } else { tpch_06 };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("tpch_06", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (lineitem,)| {
            b.iter(|| black_box(query(lineitem)))
        });
    }
}

fn benchmark_tpch_07(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let supplier = read_supplier()(&path("lineitem")).unwrap();
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let orders = read_orders()(&path("orders")).unwrap();
    let customers = read_customers()(&path("customer")).unwrap();
    let nation = read_nation()(&path("nation")).unwrap();
    let data = (supplier, lineitems, orders, customers, nation);
    for parallel in [false, true] {
        let query = if parallel { tpch_07_parallel } else { tpch_07 };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("tpch_07", format!("SF1_{variant}"));
        c.bench_with_input(
            id,
            &data,
            |b, (supplier, lineitems, orders, customers, nation)| {
                b.iter(|| black_box(query(supplier, lineitems, orders, customers, nation)))
            },
        );
    }
}

fn benchmark_tpch_08(c: &mut Criterion) {
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
        let query = if parallel { tpch_08_parallel } else { tpch_08 };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("tpch_08", format!("SF1_{variant}"));
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

fn benchmark_tpch_09(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let part = read_part()(&path("part")).unwrap();
    let supplier = read_supplier()(&path("lineitem")).unwrap();
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let partsupp = read_partsupp()(&path("partsupp")).unwrap();
    let orders = read_orders()(&path("orders")).unwrap();
    let nation = read_nation()(&path("nation")).unwrap();
    let data = (part, supplier, lineitems, partsupp, orders, nation);
    for parallel in [false, true] {
        let query = if parallel { tpch_09_parallel } else { tpch_09 };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("tpch_09", format!("SF1_{variant}"));
        c.bench_with_input(
            id,
            &data,
            |b, (part, supplier, lineitems, partsupp, orders, nation)| {
                b.iter(|| black_box(query(part, supplier, lineitems, partsupp, orders, nation)))
            },
        );
    }
}

fn benchmark_tpch_10(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let customers = read_customers()(&path("customer")).unwrap();
    let orders = read_orders()(&path("orders")).unwrap();
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let nation = read_nation()(&path("nation")).unwrap();
    let data = (customers, orders, lineitems, nation);
    for parallel in [false, true] {
        let query = if parallel { tpch_10_parallel } else { tpch_10 };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("tpch_10", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (customers, orders, lineitems, nation)| {
            b.iter(|| black_box(query(customers, orders, lineitems, nation)))
        });
    }
}

fn benchmark_tpch_11(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let supplier = read_supplier()(&path("supplier")).unwrap();
    let partsupp = read_partsupp()(&path("partsupp")).unwrap();
    let nation = read_nation()(&path("nation")).unwrap();
    let data = (supplier, partsupp, nation);
    for parallel in [false, true] {
        let query = if parallel { tpch_11_parallel } else { tpch_11 };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("tpch_11", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (supplier, partsupp, nation)| {
            b.iter(|| black_box(query(supplier, partsupp, nation)))
        });
    }
}

fn benchmark_tpch_12(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let orders = read_orders()(&path("orders")).unwrap();
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let data = (orders, lineitems);
    for parallel in [false, true] {
        let query = if parallel { tpch_12_parallel } else { tpch_12 };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("tpch_12", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (orders, lineitem)| {
            b.iter(|| black_box(query(orders, lineitem)))
        });
    }
}

fn benchmark_tpch_13(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let orders = read_orders()(&path("orders")).unwrap();
    let customer = read_customers()(&path("customer")).unwrap();
    let data = (orders, customer);
    for parallel in [false, true] {
        let query = if parallel { tpch_13_parallel } else { tpch_13 };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("tpch_13", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (orders, customer)| {
            b.iter(|| black_box(query(orders, customer)))
        });
    }
}

fn benchmark_tpch_14(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let part = read_part()(&path("part")).unwrap();
    let data = (lineitems, part);
    for parallel in [false, true] {
        let query = if parallel { tpch_14_parallel } else { tpch_14 };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("tpch_14", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (lineitems, part)| {
            b.iter(|| black_box(query(lineitems, part)))
        });
    }
}

fn benchmark_tpch_15(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let supplier = read_supplier()(&path("supplier")).unwrap();
    let data = (lineitems, supplier);
    for parallel in [false, true] {
        let query = if parallel { tpch_15_parallel } else { tpch_15 };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("tpch_15", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (lineitems, supplier)| {
            b.iter(|| black_box(query(lineitems, supplier)))
        });
    }
}

fn benchmark_tpch_16(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let partsupp = read_partsupp()(&path("partsupp")).unwrap();
    let part = read_part()(&path("part")).unwrap();
    let supplier = read_supplier()(&path("supplier")).unwrap();
    let data = (partsupp, part, supplier);
    for parallel in [false, true] {
        let query = if parallel { tpch_16_parallel } else { tpch_16 };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("tpch_16", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (partsupp, part, supplier)| {
            b.iter(|| black_box(query(partsupp, part, supplier)))
        });
    }
}

fn benchmark_tpch_17(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let part = read_part()(&path("part")).unwrap();
    let data = (lineitems, part);
    for parallel in [false, true] {
        let query = if parallel { tpch_17_parallel } else { tpch_17 };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("tpch_17", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (lineitems, part)| {
            b.iter(|| black_box(query(lineitems, part)))
        });
    }
}

fn benchmark_tpch_18(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let customers = read_customers()(&path("customer")).unwrap();
    let orders = read_orders()(&path("orders")).unwrap();
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let data = (customers, orders, lineitems);
    for parallel in [false, true] {
        let query = if parallel { tpch_18_parallel } else { tpch_18 };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("tpch_18", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (customers, orders, lineitems)| {
            b.iter(|| black_box(query(customers, orders, lineitems)))
        });
    }
}

fn benchmark_tpch_19(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let part = read_part()(&path("part")).unwrap();
    let data = (lineitems, part);
    for parallel in [false, true] {
        let query = if parallel { tpch_19_parallel } else { tpch_19 };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("tpch_19", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (lineitems, part)| {
            b.iter(|| black_box(query(lineitems, part)))
        });
    }
}

fn benchmark_tpch_20(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let supplier = read_supplier()(&path("supplier")).unwrap();
    let nation = read_nation()(&path("nation")).unwrap();
    let part = read_part()(&path("part")).unwrap();
    let partsupp = read_partsupp()(&path("partsupp")).unwrap();
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let data = (supplier, nation, part, partsupp, lineitems);
    for parallel in [false, true] {
        let query = if parallel { tpch_20_parallel } else { tpch_20 };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("tpch_20", format!("SF1_{variant}"));
        c.bench_with_input(
            id,
            &data,
            |b, (supplier, nation, part, partsupp, lineitems)| {
                b.iter(|| black_box(query(supplier, nation, part, partsupp, lineitems)))
            },
        );
    }
}

fn benchmark_tpch_21(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let supplier = read_supplier()(&path("lineitem")).unwrap();
    let lineitems = read_lineitems()(&path("lineitem")).unwrap();
    let orders = read_orders()(&path("orders")).unwrap();
    let nation = read_nation()(&path("nation")).unwrap();
    let data = (supplier, lineitems, orders, nation);
    // FIXME
    //  for parallel in [false, true] {
    for parallel in [false] {
        let query = if parallel { tpch_21_parallel } else { tpch_21 };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("tpch_21", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (supplier, lineitems, orders, nation)| {
            b.iter(|| black_box(query(supplier, lineitems, orders, nation)))
        });
    }
}

fn benchmark_tpch_22(c: &mut Criterion) {
    let path = |table| format!("datasets/tpch_datasets/SF_1/{table}.tbl");
    let customer = read_customers()(&path("customer")).unwrap();
    let orders = read_orders()(&path("orders")).unwrap();
    let data = (customer, orders);
    for parallel in [false, true] {
        let query = if parallel { tpch_22_parallel } else { tpch_22 };
        let variant = if parallel { "parallel" } else { "sequential" };
        let id = BenchmarkId::new("tpch_22", format!("SF1_{variant}"));
        c.bench_with_input(id, &data, |b, (customer, orders)| {
            b.iter(|| black_box(query(customer, orders)))
        });
    }
}

criterion_group!(
    benches,
    benchmark_tpch_01,
    benchmark_tpch_02,
    benchmark_tpch_03,
    benchmark_tpch_04,
    benchmark_tpch_05,
    benchmark_tpch_06,
    benchmark_tpch_07,
    benchmark_tpch_08,
    benchmark_tpch_09,
    benchmark_tpch_10,
    benchmark_tpch_11,
    benchmark_tpch_12,
    benchmark_tpch_13,
    benchmark_tpch_14,
    benchmark_tpch_15,
    benchmark_tpch_16,
    benchmark_tpch_17,
    benchmark_tpch_18,
    benchmark_tpch_19,
    benchmark_tpch_20,
    benchmark_tpch_21,
    benchmark_tpch_22,
);
criterion_main!(benches);
