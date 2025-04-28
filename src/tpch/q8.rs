#![allow(unused_mut, unused_variables)]

use super::types::*;
// use rayon::prelude::*;
use sdql_runtime::*;

pub fn q8_query(
    part: &Part,
    supplier: &Supplier,
    lineitem: &Lineitem,
    orders: &Orders,
    customer: &Customer,
    nation: &Nation,
    region: &Region,
) -> TypeQ8 {
    let mut r_h: HashMap<i32, Record<(i32,)>> = (0..region.3)
        .into_iter()
        .filter(|&i| region.1[i as usize] == VarChar::from_str("AMERICA").unwrap())
        .map(|i| (region.0[i as usize], Record::new((region.0[i as usize],))))
        .collect();
    let mut n_h: HashMap<i32, i32> = (0..nation.4)
        .into_iter()
        .filter(|&i| r_h.contains_key(&nation.2[i as usize]))
        .map(|i| (nation.0[i as usize], 1i32))
        .collect();
    let mut nationkey_to_name: HashMap<i32, Record<(VarChar<25>,)>> = (0..nation.4)
        .into_iter()
        .map(|i| (nation.0[i as usize], Record::new((nation.1[i as usize],))))
        .collect();
    let mut s_h: HashMap<i32, Record<(i32,)>> = (0..supplier.7)
        .into_iter()
        .map(|i| {
            (
                supplier.0[i as usize],
                Record::new((supplier.3[i as usize],)),
            )
        })
        .collect();
    let mut c_h: Vec<i32> =
        (0..customer.8)
            .into_iter()
            .fold(vec![i32::default(); 200001], |mut acc: Vec<i32>, i| {
                acc[customer.0[i as usize] as usize] += customer.3[i as usize];
                acc
            });
    let mut p_h: HashMap<i32, Record<(i32,)>> = (0..part.9)
        .into_iter()
        .filter(|&i| part.4[i as usize] == VarChar::from_str("ECONOMY ANODIZED STEEL").unwrap())
        .map(|i| (part.0[i as usize], Record::new((part.0[i as usize],))))
        .collect();
    let mut o_h: HashMap<i32, Record<(i32, Date)>> = (0..orders.9)
        .into_iter()
        .filter(|&i| {
            date!(19950101) <= orders.4[i as usize] && orders.4[i as usize] <= date!(19961231)
        })
        .map(|i| {
            (
                orders.0[i as usize],
                Record::new((orders.1[i as usize], orders.4[i as usize])),
            )
        })
        .collect();
    let mut l_h: HashMap<i32, Record<(OrderedFloat<f64>, OrderedFloat<f64>)>> = (0..lineitem.16)
        .into_iter()
        .filter(|&i| {
            p_h.contains_key(&lineitem.1[i as usize])
                && o_h.contains_key(&lineitem.0[i as usize])
                && n_h.contains_key(&c_h[o_h[&lineitem.0[i as usize]].0 as usize])
        })
        .map(|i| (i, o_h[&lineitem.0[i as usize]].1))
        .map(|(i, orderdate)| (i, orderdate, orderdate.year()))
        .map(|(i, orderdate, orderyear)| {
            (
                i,
                orderdate,
                orderyear,
                lineitem.5[i as usize] * (OrderedFloat(1f64) - lineitem.6[i as usize]),
            )
        })
        .map(|(i, orderdate, orderyear, volume)| {
            (
                i,
                orderdate,
                orderyear,
                volume,
                if nationkey_to_name[&s_h[&lineitem.2[i as usize]].0].0
                    == VarChar::from_str("BRAZIL").unwrap()
                {
                    volume
                } else {
                    OrderedFloat(0f64)
                },
            )
        })
        .fold(
            HashMap::default(),
            |mut acc: HashMap<i32, Record<(OrderedFloat<f64>, OrderedFloat<f64>)>>,
             (i, orderdate, orderyear, volume, brazil_volume)| {
                acc[&orderyear] += Record::new((brazil_volume, volume));
                acc
            },
        );
    l_h.into_iter()
        .map(|(k, v)| (Record::new((k, v.0 / v.1)), TRUE))
        .collect()
}

pub fn q8_query_rayon(
    _part: &Part,
    _supplier: &Supplier,
    _lineitem: &Lineitem,
    _orders: &Orders,
    _customer: &Customer,
    _nation: &Nation,
    _region: &Region,
) -> TypeQ8 {
    todo!()
}
