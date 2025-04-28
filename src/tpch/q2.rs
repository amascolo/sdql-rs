#![allow(unused_mut, unused_variables)]

use super::types::*;
use rayon::prelude::*;
use sdql_runtime::*;

pub fn q2_query(
    part: &Part,
    supplier: &Supplier,
    partsupp: &Partsupp,
    nation: &Nation,
    region: &Region,
) -> TypeQ2 {
    let mut l_h: HashMap<i32, Record<(i32,)>> = (0..region.3)
        .into_iter()
        .filter(|&i| region.1[i as usize] == VarChar::from_str("EUROPE").unwrap())
        .map(|i| (region.0[i as usize], Record::new((region.0[i as usize],))))
        .collect();
    let mut n_h: HashMap<i32, VarChar<25>> = (0..nation.4)
        .into_iter()
        .filter(|&i| l_h.contains_key(&nation.2[i as usize]))
        .map(|i| (nation.0[i as usize], nation.1[i as usize]))
        .collect();
    let mut s_h: HashMap<
        i32,
        Record<(
            OrderedFloat<f64>,
            VarChar<25>,
            VarChar<25>,
            VarChar<40>,
            VarChar<15>,
            VarChar<101>,
        )>,
    > = (0..supplier.7)
        .into_iter()
        .filter(|&i| n_h.contains_key(&supplier.3[i as usize]))
        .map(|i| {
            (
                supplier.0[i as usize],
                Record::new((
                    supplier.5[i as usize],
                    supplier.1[i as usize],
                    n_h[&supplier.3[i as usize]],
                    supplier.2[i as usize],
                    supplier.4[i as usize],
                    supplier.6[i as usize],
                )),
            )
        })
        .collect();
    let mut p_h: HashMap<i32, Record<(VarChar<25>,)>> = (0..part.9)
        .into_iter()
        .filter(|&i| part.5[i as usize] == 15i32 && part.4[i as usize].ends_with(&"BRASS"))
        .map(|i| (part.0[i as usize], Record::new((part.2[i as usize],))))
        .collect();
    let mut ps_h: HashMap<i32, OrderedFloat<f64>> = (0..partsupp.5)
        .into_iter()
        .filter(|&i| {
            p_h.contains_key(&partsupp.0[i as usize]) && s_h.contains_key(&partsupp.1[i as usize])
        })
        .fold(
            HashMap::default(),
            |mut acc: HashMap<i32, OrderedFloat<f64>>, i| {
                acc[&partsupp.0[i as usize]] += partsupp.3[i as usize];
                acc
            },
        );
    (0..partsupp.5)
        .into_iter()
        .filter(|&i| {
            ps_h.contains_key(&partsupp.0[i as usize])
                && ps_h[&partsupp.0[i as usize]] == partsupp.3[i as usize]
                && s_h.contains_key(&partsupp.1[i as usize])
        })
        .map(|i| {
            (
                Record::new((
                    s_h[&partsupp.1[i as usize]].0,
                    s_h[&partsupp.1[i as usize]].1,
                    s_h[&partsupp.1[i as usize]].2,
                    partsupp.0[i as usize],
                    p_h[&partsupp.0[i as usize]].0,
                    s_h[&partsupp.1[i as usize]].4,
                    s_h[&partsupp.1[i as usize]].3,
                    s_h[&partsupp.1[i as usize]].5,
                )),
                TRUE,
            )
        })
        .collect()
}

pub fn q2_query_rayon(
    part: &Part,
    supplier: &Supplier,
    partsupp: &Partsupp,
    nation: &Nation,
    region: &Region,
) -> TypeQ2 {
    let mut l_h: HashMap<i32, Record<(i32,)>> = (0..region.3)
        .into_par_iter()
        .filter(|&i| region.1[i as usize] == VarChar::from_str("EUROPE").unwrap())
        .map(|i| (region.0[i as usize], Record::new((region.0[i as usize],))))
        .collect();
    let mut n_h: HashMap<i32, VarChar<25>> = (0..nation.4)
        .into_par_iter()
        .filter(|&i| l_h.contains_key(&nation.2[i as usize]))
        .map(|i| (nation.0[i as usize], nation.1[i as usize]))
        .collect();
    let mut s_h: HashMap<
        i32,
        Record<(
            OrderedFloat<f64>,
            VarChar<25>,
            VarChar<25>,
            VarChar<40>,
            VarChar<15>,
            VarChar<101>,
        )>,
    > = (0..supplier.7)
        .into_par_iter()
        .filter(|&i| n_h.contains_key(&supplier.3[i as usize]))
        .map(|i| {
            (
                supplier.0[i as usize],
                Record::new((
                    supplier.5[i as usize],
                    supplier.1[i as usize],
                    n_h[&supplier.3[i as usize]],
                    supplier.2[i as usize],
                    supplier.4[i as usize],
                    supplier.6[i as usize],
                )),
            )
        })
        .collect();
    let mut p_h: HashMap<i32, Record<(VarChar<25>,)>> = (0..part.9)
        .into_par_iter()
        .filter(|&i| part.5[i as usize] == 15i32 && part.4[i as usize].ends_with(&"BRASS"))
        .map(|i| (part.0[i as usize], Record::new((part.2[i as usize],))))
        .collect();
    let mut ps_h: HashMap<i32, OrderedFloat<f64>> = (0..partsupp.5)
        .into_par_iter()
        .filter(|&i| {
            p_h.contains_key(&partsupp.0[i as usize]) && s_h.contains_key(&partsupp.1[i as usize])
        })
        .fold(
            HashMap::default,
            |mut acc: HashMap<i32, OrderedFloat<f64>>, i| {
                acc[&partsupp.0[i as usize]] += partsupp.3[i as usize];
                acc
            },
        )
        .sum();
    (0..partsupp.5)
        .into_par_iter()
        .filter(|&i| {
            ps_h.contains_key(&partsupp.0[i as usize])
                && ps_h[&partsupp.0[i as usize]] == partsupp.3[i as usize]
                && s_h.contains_key(&partsupp.1[i as usize])
        })
        .map(|i| {
            (
                Record::new((
                    s_h[&partsupp.1[i as usize]].0,
                    s_h[&partsupp.1[i as usize]].1,
                    s_h[&partsupp.1[i as usize]].2,
                    partsupp.0[i as usize],
                    p_h[&partsupp.0[i as usize]].0,
                    s_h[&partsupp.1[i as usize]].4,
                    s_h[&partsupp.1[i as usize]].3,
                    s_h[&partsupp.1[i as usize]].5,
                )),
                TRUE,
            )
        })
        .collect()
}
