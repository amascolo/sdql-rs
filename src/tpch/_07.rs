#![allow(unused_mut, unused_variables)]

use super::types::*;
use rayon::prelude::*;
use sdql_runtime::*;

pub fn tpch_07(
    supplier: &Supplier,
    lineitem: &Lineitem,
    orders: &Orders,
    customer: &Customer,
    nation: &Nation,
) -> TypeQ7 {
    let mut nationkey_to_name: HashMap<i32, Record<(VarChar<25>,)>> = (0..nation.4)
        .into_iter()
        .filter(|&i| {
            nation.1[i as usize] == VarChar::from_str("FRANCE").unwrap()
                || nation.1[i as usize] == VarChar::from_str("GERMANY").unwrap()
        })
        .map(|i| (nation.0[i as usize], Record::new((nation.1[i as usize],))))
        .collect();
    let mut custkey_to_name: HashMap<i32, VarChar<25>> = (0..customer.8)
        .into_iter()
        .filter(|&i| nationkey_to_name.contains_key(&customer.3[i as usize]))
        .map(|i| {
            (
                customer.0[i as usize],
                nationkey_to_name[&customer.3[i as usize]].0,
            )
        })
        .collect();
    let mut orderkey_to_name: HashMap<i32, VarChar<25>> = (0..orders.9)
        .into_iter()
        .filter(|&i| custkey_to_name.contains_key(&orders.1[i as usize]))
        .map(|i| (orders.0[i as usize], custkey_to_name[&orders.1[i as usize]]))
        .collect();
    let mut suppkey_to_name: HashMap<i32, VarChar<25>> = (0..supplier.7)
        .into_iter()
        .filter(|&i| nationkey_to_name.contains_key(&supplier.3[i as usize]))
        .map(|i| {
            (
                supplier.0[i as usize],
                nationkey_to_name[&supplier.3[i as usize]].0,
            )
        })
        .collect();
    let mut l_h: HashMap<Record<(VarChar<25>, VarChar<25>, i32)>, Record<(OrderedFloat<f64>,)>> =
        (0..lineitem.16)
            .into_iter()
            .filter(|&i| {
                date!(19950101) <= lineitem.10[i as usize]
                    && lineitem.10[i as usize] <= date!(19961231)
                    && orderkey_to_name.contains_key(&lineitem.0[i as usize])
                    && suppkey_to_name.contains_key(&lineitem.2[i as usize])
                    && (orderkey_to_name[&lineitem.0[i as usize]]
                        == VarChar::from_str("FRANCE").unwrap()
                        && suppkey_to_name[&lineitem.2[i as usize]]
                            == VarChar::from_str("GERMANY").unwrap()
                        || orderkey_to_name[&lineitem.0[i as usize]]
                            == VarChar::from_str("GERMANY").unwrap()
                            && suppkey_to_name[&lineitem.2[i as usize]]
                                == VarChar::from_str("FRANCE").unwrap())
            })
            .fold(
                HashMap::default(),
                |mut acc: HashMap<
                    Record<(VarChar<25>, VarChar<25>, i32)>,
                    Record<(OrderedFloat<f64>,)>,
                >,
                 i| {
                    acc[&Record::new((
                        suppkey_to_name[&lineitem.2[i as usize]],
                        orderkey_to_name[&lineitem.0[i as usize]],
                        lineitem.10[i as usize].year(),
                    ))] += Record::new((
                        lineitem.5[i as usize] * (OrderedFloat(1f64) - lineitem.6[i as usize]),
                    ));
                    acc
                },
            );
    l_h.into_iter()
        .map(|(k, v)| (Record::new((k.0, k.1, k.2, v.0)), TRUE))
        .collect()
}

pub fn tpch_07_parallel(
    supplier: &Supplier,
    lineitem: &Lineitem,
    orders: &Orders,
    customer: &Customer,
    nation: &Nation,
) -> TypeQ7 {
    let mut nationkey_to_name: HashMap<i32, Record<(VarChar<25>,)>> = (0..nation.4)
        .into_par_iter()
        .filter(|&i| {
            nation.1[i as usize] == VarChar::from_str("FRANCE").unwrap()
                || nation.1[i as usize] == VarChar::from_str("GERMANY").unwrap()
        })
        .map(|i| (nation.0[i as usize], Record::new((nation.1[i as usize],))))
        .collect();
    let mut custkey_to_name: HashMap<i32, VarChar<25>> = (0..customer.8)
        .into_par_iter()
        .filter(|&i| nationkey_to_name.contains_key(&customer.3[i as usize]))
        .map(|i| {
            (
                customer.0[i as usize],
                nationkey_to_name[&customer.3[i as usize]].0,
            )
        })
        .collect();
    let mut orderkey_to_name: HashMap<i32, VarChar<25>> = (0..orders.9)
        .into_par_iter()
        .filter(|&i| custkey_to_name.contains_key(&orders.1[i as usize]))
        .map(|i| (orders.0[i as usize], custkey_to_name[&orders.1[i as usize]]))
        .collect();
    let mut suppkey_to_name: HashMap<i32, VarChar<25>> = (0..supplier.7)
        .into_par_iter()
        .filter(|&i| nationkey_to_name.contains_key(&supplier.3[i as usize]))
        .map(|i| {
            (
                supplier.0[i as usize],
                nationkey_to_name[&supplier.3[i as usize]].0,
            )
        })
        .collect();
    let mut l_h: HashMap<Record<(VarChar<25>, VarChar<25>, i32)>, Record<(OrderedFloat<f64>,)>> =
        (0..lineitem.16)
            .into_par_iter()
            .filter(|&i| {
                date!(19950101) <= lineitem.10[i as usize]
                    && lineitem.10[i as usize] <= date!(19961231)
                    && orderkey_to_name.contains_key(&lineitem.0[i as usize])
                    && suppkey_to_name.contains_key(&lineitem.2[i as usize])
                    && (orderkey_to_name[&lineitem.0[i as usize]]
                        == VarChar::from_str("FRANCE").unwrap()
                        && suppkey_to_name[&lineitem.2[i as usize]]
                            == VarChar::from_str("GERMANY").unwrap()
                        || orderkey_to_name[&lineitem.0[i as usize]]
                            == VarChar::from_str("GERMANY").unwrap()
                            && suppkey_to_name[&lineitem.2[i as usize]]
                                == VarChar::from_str("FRANCE").unwrap())
            })
            .fold(
                HashMap::default,
                |mut acc: HashMap<
                    Record<(VarChar<25>, VarChar<25>, i32)>,
                    Record<(OrderedFloat<f64>,)>,
                >,
                 i| {
                    acc[&Record::new((
                        suppkey_to_name[&lineitem.2[i as usize]],
                        orderkey_to_name[&lineitem.0[i as usize]],
                        lineitem.10[i as usize].year(),
                    ))] += Record::new((
                        lineitem.5[i as usize] * (OrderedFloat(1f64) - lineitem.6[i as usize]),
                    ));
                    acc
                },
            )
            .sum();
    l_h.into_par_iter()
        .map(|(k, v)| (Record::new((k.0, k.1, k.2, v.0)), TRUE))
        .collect()
}
