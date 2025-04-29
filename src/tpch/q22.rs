#![allow(unused_mut, unused_variables)]

use super::types::*;
// use rayon::prelude::*;
use sdql_runtime::*;

pub fn q22_query(customer: &Customer, orders: &Orders) -> TypeQ22 {
    let mut o_h: Vec<i32> =
        (0..orders.9)
            .into_iter()
            .fold(vec![i32::default(); 150001], |mut acc: Vec<i32>, i| {
                acc[orders.1[i as usize] as usize] += 1i32;
                acc
            });
    let mut fused: Record<(OrderedFloat<f64>, OrderedFloat<f64>)> = (0..customer.8)
        .into_iter()
        .map(|i| {
            (
                i,
                OrderedFloat(0f64) < customer.5[i as usize]
                    && (customer.4[i as usize].starts_with(&"13")
                        || customer.4[i as usize].starts_with(&"31")
                        || customer.4[i as usize].starts_with(&"23")
                        || customer.4[i as usize].starts_with(&"29")
                        || customer.4[i as usize].starts_with(&"30")
                        || customer.4[i as usize].starts_with(&"18")
                        || customer.4[i as usize].starts_with(&"17")),
            )
        })
        .map(|(i, cond)| {
            Record::new((
                if cond {
                    customer.5[i as usize]
                } else {
                    OrderedFloat(0f64)
                },
                if cond {
                    OrderedFloat(1f64)
                } else {
                    OrderedFloat(0f64)
                },
            ))
        })
        .sum();
    let mut avg: OrderedFloat<f64> = fused.0 / fused.1;
    let mut res: HashMap<Record<(VarChar<2>,)>, Record<(i32, OrderedFloat<f64>)>> = (0..customer.8)
        .into_iter()
        .filter(|&i| {
            avg < customer.5[i as usize]
                && o_h[customer.0[i as usize] as usize] == 0i32
                && (customer.4[i as usize].starts_with(&"13")
                    || customer.4[i as usize].starts_with(&"31")
                    || customer.4[i as usize].starts_with(&"23")
                    || customer.4[i as usize].starts_with(&"29")
                    || customer.4[i as usize].starts_with(&"30")
                    || customer.4[i as usize].starts_with(&"18")
                    || customer.4[i as usize].starts_with(&"17"))
        })
        .fold(
            HashMap::default(),
            |mut acc: HashMap<Record<(VarChar<2>,)>, Record<(i32, OrderedFloat<f64>)>>, i| {
                acc[&Record::new((VarChar::<{ 2usize - 0usize }>::from(
                    &(customer.4[i as usize])[0usize..2usize],
                )
                .unwrap(),))] += Record::new((1i32, customer.5[i as usize]));
                acc
            },
        );
    res.into_iter()
        .map(|(k, v)| (Record::new((k.0, v.0, v.1)), TRUE))
        .collect()
}

pub fn q22_query_rayon(customer: &Customer, orders: &Orders) -> TypeQ22 {
    todo!()
}
