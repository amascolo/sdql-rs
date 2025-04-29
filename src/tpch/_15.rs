#![allow(unused_mut, unused_variables)]

use super::types::*;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use sdql_runtime::*;

pub fn tpch_15(lineitem: &Lineitem, supplier: &Supplier) -> TypeQ15 {
    let mut suppkey_to_revenue: HashMap<i32, OrderedFloat<f64>> = (0..lineitem.16)
        .into_iter()
        .filter(|&i| {
            date!(19960101) <= lineitem.10[i as usize] && lineitem.10[i as usize] < date!(19960401)
        })
        .fold(
            HashMap::default(),
            |mut acc: HashMap<i32, OrderedFloat<f64>>, i| {
                acc[&lineitem.2[i as usize]] +=
                    lineitem.5[i as usize] * (OrderedFloat(1f64) - lineitem.6[i as usize]);
                acc
            },
        );
    let mut max_revenue: OrderedFloat<f64> = OrderedFloat(1161099.4635999997f64);
    let mut suppkey_to_supp: HashMap<i32, Record<(VarChar<25>, VarChar<40>, VarChar<15>)>> = (0
        ..supplier.7)
        .into_iter()
        .map(|i| {
            (
                supplier.0[i as usize],
                Record::new((
                    supplier.1[i as usize],
                    supplier.2[i as usize],
                    supplier.4[i as usize],
                )),
            )
        })
        .collect();
    suppkey_to_revenue
        .into_iter()
        .filter(|&(suppkey, revenue)| revenue == max_revenue)
        .map(|(suppkey, revenue)| {
            (
                Record::new((
                    suppkey,
                    suppkey_to_supp[&suppkey].0,
                    suppkey_to_supp[&suppkey].1,
                    suppkey_to_supp[&suppkey].2,
                    revenue,
                )),
                TRUE,
            )
        })
        .collect()
}

pub fn tpch_15_parallel(lineitem: &Lineitem, supplier: &Supplier) -> TypeQ15 {
    let mut suppkey_to_revenue: HashMap<i32, OrderedFloat<f64>> = (0..lineitem.16)
        .into_par_iter()
        .filter(|&i| {
            date!(19960101) <= lineitem.10[i as usize] && lineitem.10[i as usize] < date!(19960401)
        })
        .fold(
            HashMap::default,
            |mut acc: HashMap<i32, OrderedFloat<f64>>, i| {
                acc[&lineitem.2[i as usize]] +=
                    lineitem.5[i as usize] * (OrderedFloat(1f64) - lineitem.6[i as usize]);
                acc
            },
        )
        .sum();
    let mut max_revenue: OrderedFloat<f64> = OrderedFloat(1772627.2087f64);
    let mut suppkey_to_supp: HashMap<i32, Record<(VarChar<25>, VarChar<40>, VarChar<15>)>> = (0
        ..supplier.7)
        .into_par_iter()
        .map(|i| {
            (
                supplier.0[i as usize],
                Record::new((
                    supplier.1[i as usize],
                    supplier.2[i as usize],
                    supplier.4[i as usize],
                )),
            )
        })
        .collect();
    suppkey_to_revenue
        .into_iter()
        .filter(|&(suppkey, revenue)| revenue == max_revenue)
        .map(|(suppkey, revenue)| {
            (
                Record::new((
                    suppkey,
                    suppkey_to_supp[&suppkey].0,
                    suppkey_to_supp[&suppkey].1,
                    suppkey_to_supp[&suppkey].2,
                    revenue,
                )),
                TRUE,
            )
        })
        .collect()
}
