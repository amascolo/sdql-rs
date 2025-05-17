#![allow(unused_mut, unused_variables)]

use super::types::*;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use sdql_runtime::*;

pub fn tpch_22(customer: &Customer, orders: &Orders) -> TypeQ22 {
    // TODO update code generator
    // let mut o_h: Vec<Bool> =
    //     (0..orders.9)
    //         .into_iter()
    //         .fold(vec![Bool::default(); 150001], |mut acc: Vec<Bool>, i| {
    //             acc[orders.1[i as usize] as usize] += TRUE;
    //             acc
    //         });
    let mut o_h = vec![Bool::default(); 150001];
    // TODO this also works - but sequential only (unless you use AtomicBool + Ordering::Relaxed)
    // (0..orders.9).into_iter().for_each(|i| {
    //     *o_h[orders.1[i as usize] as usize] = *TRUE;
    // });
    let ptr = o_h.as_mut_ptr() as usize;
    (0..orders.9).into_iter().for_each(move |i| unsafe {
        *(ptr as *mut Bool).add(orders.1[i as usize] as usize) = TRUE;
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
                && o_h[customer.0[i as usize] as usize] == FALSE
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

pub fn tpch_22_parallel(customer: &Customer, orders: &Orders) -> TypeQ22 {
    // TODO experiment - consider switching to paradis if we can get it to work with i32
    // use paradis::index::{narrow_access, IndexList};
    // use paradis::rayon::create_par_iter;
    // let orders = (
    //     (),
    //     orders
    //         .1
    //         .iter()
    //         .copied()
    //         .map(|i| i as usize)
    //         .collect::<Vec<_>>(),
    // );
    // // let indices = orders.1.check_unique().unwrap();
    // let indices = unsafe { orders.1.assume_unique() };
    // let access = narrow_access(o_h.as_mut_slice(), &indices).unwrap();
    // create_par_iter(access).for_each(|flag| *flag = TRUE);
    // TODO update code generator
    let mut o_h = vec![Bool::default(); 150001];
    let ptr = o_h.as_mut_ptr() as usize;
    (0..orders.9).into_par_iter().for_each(move |i| unsafe {
        *(ptr as *mut Bool).add(orders.1[i as usize] as usize) = TRUE;
    });
    let mut fused: Record<(OrderedFloat<f64>, OrderedFloat<f64>)> = (0..customer.8)
        .into_par_iter()
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
        .into_par_iter()
        .filter(|&i| {
            avg < customer.5[i as usize]
                && o_h[customer.0[i as usize] as usize] == FALSE
                && (customer.4[i as usize].starts_with(&"13")
                    || customer.4[i as usize].starts_with(&"31")
                    || customer.4[i as usize].starts_with(&"23")
                    || customer.4[i as usize].starts_with(&"29")
                    || customer.4[i as usize].starts_with(&"30")
                    || customer.4[i as usize].starts_with(&"18")
                    || customer.4[i as usize].starts_with(&"17"))
        })
        .fold(
            HashMap::default,
            |mut acc: HashMap<Record<(VarChar<2>,)>, Record<(i32, OrderedFloat<f64>)>>, i| {
                acc[&Record::new((VarChar::<{ 2usize - 0usize }>::from(
                    &(customer.4[i as usize])[0usize..2usize],
                )
                .unwrap(),))] += Record::new((1i32, customer.5[i as usize]));
                acc
            },
        )
        .sum();
    res.into_par_iter()
        .map(|(k, v)| (Record::new((k.0, v.0, v.1)), TRUE))
        .collect()
}
