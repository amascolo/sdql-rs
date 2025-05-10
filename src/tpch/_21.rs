#![allow(unused_mut, unused_variables)]

use super::types::*;
// use rayon::prelude::*;
use sdql_runtime::*;

pub fn tpch_21(
    supplier: &Supplier,
    lineitem: &Lineitem,
    orders: &Orders,
    nation: &Nation,
) -> TypeQ21 {
    let mut nation_indexed: HashMap<i32, Record<(i32,)>> = (0..nation.4)
        .into_iter()
        .filter(|&i| nation.1[i as usize] == VarChar::from_str("SAUDI ARABIA").unwrap())
        .map(|i| (nation.0[i as usize], Record::new((nation.0[i as usize],))))
        .collect();
    let mut su_probed: HashMap<i32, VarChar<25>> = (0..supplier.7)
        .into_iter()
        .filter(|&i| nation_indexed.contains_key(&supplier.3[i as usize]))
        .map(|i| (supplier.0[i as usize], supplier.1[i as usize]))
        .collect();
    // TODO update code generator
    // let mut ord_indexed: Vec<Bool> = (0..orders.9)
    //     .into_iter()
    //     .filter(|&i| orders.2[i as usize] == VarChar::from_str("F").unwrap())
    //     .fold(vec![Bool::default(); 6000001], |mut acc: Vec<Bool>, i| {
    //         acc[orders.0[i as usize] as usize] += TRUE;
    //         acc
    //     });
    let mut ord_indexed: Vec<Bool> = vec![Bool::default(); 6000001];
    let ptr = ord_indexed.as_mut_ptr() as usize;
    (0..orders.9)
        .into_iter()
        .filter(|&i| orders.2[i as usize] == VarChar::from_str("F").unwrap())
        .for_each(move |i| unsafe {
            *(ptr as *mut Bool).add(orders.0[i as usize] as usize) = TRUE;
        });
    // TODO update code generator
    // let mut l2_indexed: Vec<SmallVecDict<[i32; 4usize]>> = (0..lineitem.16).into_iter().fold(
    //     vec![SmallVecDict::default(); 6000001],
    //     |mut acc: Vec<SmallVecDict<[i32; 4usize]>>, i| {
    //         acc[lineitem.0[i as usize] as usize][lineitem.2[i as usize]] += 1i32;
    //         acc
    //     },
    // );
    let mut l2_indexed: Vec<SmallVecDict<[i32; 4usize]>> = vec![SmallVecDict::default(); 6000001];
    let ptr = l2_indexed.as_mut_ptr() as usize;
    (0..lineitem.16).into_iter().for_each(move |i| unsafe {
        (&mut (*(ptr as *mut SmallVecDict<[i32; 4usize]>).add(lineitem.0[i as usize] as usize)))
            [lineitem.2[i as usize]] += 1;
    });
    // TODO update code generator
    // let mut l3_indexed: Vec<SmallVecDict<[i32; 4usize]>> = (0..lineitem.16)
    //     .into_iter()
    //     .filter(|&i| lineitem.11[i as usize] < lineitem.12[i as usize])
    //     .fold(
    //         vec![SmallVecDict::default(); 6000001],
    //         |mut acc: Vec<SmallVecDict<[i32; 4usize]>>, i| {
    //             acc[lineitem.0[i as usize] as usize][lineitem.2[i as usize]] += 1i32;
    //             acc
    //         },
    //     );
    let mut l3_indexed: Vec<SmallVecDict<[i32; 4usize]>> = vec![SmallVecDict::default(); 6000001];
    let ptr = l3_indexed.as_mut_ptr() as usize;
    (0..lineitem.16)
        .into_iter()
        .filter(|&i| lineitem.11[i as usize] < lineitem.12[i as usize])
        .for_each(move |i| unsafe {
            (&mut *(ptr as *mut SmallVecDict<[i32; 4usize]>)
                .add(lineitem.0[i as usize] as usize))[lineitem.2[i as usize]] += 1;
        });
    let mut l1_probed: HashMap<Record<(VarChar<25>,)>, Record<(i32,)>> = (0..lineitem.16)
        .into_iter()
        .filter(|&i| {
            lineitem.11[i as usize] < lineitem.12[i as usize]
                && su_probed.contains_key(&lineitem.2[i as usize])
                && ord_indexed[lineitem.0[i as usize] as usize] == TRUE
                && 1i32 < l2_indexed[lineitem.0[i as usize] as usize].len() as i32
                && l3_indexed[lineitem.0[i as usize] as usize].len() as i32 <= 1i32
        })
        .fold(
            HashMap::default(),
            |mut acc: HashMap<Record<(VarChar<25>,)>, Record<(i32,)>>, i| {
                acc[&Record::new((su_probed[&lineitem.2[i as usize]],))] += Record::new((1i32,));
                acc
            },
        );
    l1_probed
        .into_iter()
        .map(|(k, v)| (Record::new((k.0, v.0)), TRUE))
        .collect()
}

pub fn tpch_21_parallel(
    supplier: &Supplier,
    lineitem: &Lineitem,
    orders: &Orders,
    nation: &Nation,
) -> TypeQ21 {
    todo!()
    // let mut nation_indexed: HashMap<i32, Record<(i32,)>> = (0..nation.4)
    //     .into_par_iter()
    //     .filter(|&i| nation.1[i as usize] == VarChar::from_str("SAUDI ARABIA").unwrap())
    //     .map(|i| (nation.0[i as usize], Record::new((nation.0[i as usize],))))
    //     .collect();
    // let mut su_probed: HashMap<i32, VarChar<25>> = (0..supplier.7)
    //     .into_par_iter()
    //     .filter(|&i| nation_indexed.contains_key(&supplier.3[i as usize]))
    //     .map(|i| (supplier.0[i as usize], supplier.1[i as usize]))
    //     .collect();
    // // TODO update code generator
    // let mut ord_indexed: Vec<Bool> = vec![Bool::default(); 6000001];
    // let ptr = ord_indexed.as_mut_ptr() as usize;
    // (0..orders.9)
    //     .into_par_iter()
    //     .filter(|&i| orders.2[i as usize] == VarChar::from_str("F").unwrap())
    //     .for_each(move |i| unsafe {
    //         *(ptr as *mut Bool).add(orders.0[i as usize] as usize) = TRUE;
    //     });
    // // FIXME would require SmallVecDict to be thread-safe - runtime crash on borrow_mut()
    // //  (note: sdqlpy's C++ solution used tbb::concurrent_vector for the inner vector)
    // let mut l2_indexed: Vec<SmallVecDict<[i32; 4usize]>> = vec![SmallVecDict::default(); 6000001];
    // let ptr = l2_indexed.as_mut_ptr() as usize;
    // (0..lineitem.16).into_par_iter().for_each(|i| unsafe {
    //     (&mut (*(ptr as *mut SmallVecDict<[i32; 4usize]>).add(lineitem.0[i as usize] as usize)))
    //         [lineitem.2[i as usize]] += 1;
    // });
    // // FIXME would require SmallVecDict to be thread-safe - runtime crash on borrow_mut()
    // //  (note: sdqlpy's C++ solution used tbb::concurrent_vector for the inner vector)
    // let mut l3_indexed: Vec<SmallVecDict<[i32; 4usize]>> = vec![SmallVecDict::default(); 6000001];
    // let ptr = l3_indexed.as_mut_ptr() as usize;
    // (0..lineitem.16)
    //     .into_par_iter()
    //     .filter(|&i| lineitem.11[i as usize] < lineitem.12[i as usize])
    //     .for_each(|i| unsafe {
    //         (&mut *(ptr as *mut SmallVecDict<[i32; 4usize]>)
    //             .add(lineitem.0[i as usize] as usize))[lineitem.2[i as usize]] += 1;
    //     });
    // // FIXME would require SmallVecDict to be thread-safe - won't compile
    // let mut l1_probed: HashMap<Record<(VarChar<25>,)>, Record<(i32,)>> = (0..lineitem.16)
    //     .into_par_iter()
    //     .filter(|&i| {
    //         lineitem.11[i as usize] < lineitem.12[i as usize]
    //             && su_probed.contains_key(&lineitem.2[i as usize])
    //             && ord_indexed[lineitem.0[i as usize] as usize] == TRUE
    //             && 1i32 < l2_indexed[lineitem.0[i as usize] as usize].len() as i32
    //             && l3_indexed[lineitem.0[i as usize] as usize].len() as i32 <= 1i32
    //     })
    //     .fold(
    //         HashMap::default,
    //         |mut acc: HashMap<Record<(VarChar<25>,)>, Record<(i32,)>>, i| {
    //             acc[&Record::new((su_probed[&lineitem.2[i as usize]],))] += Record::new((1i32,));
    //             acc
    //         },
    //     )
    //     .sum();
    // l1_probed
    //     .into_par_iter()
    //     .map(|(k, v)| (Record::new((k.0, v.0)), TRUE))
    //     .collect()
}
