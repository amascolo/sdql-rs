#![allow(unused_mut, unused_variables)]

use super::types::*;
use rayon::prelude::*;
use sdql_runtime::*;

pub fn tpch_11(supplier: &Supplier, partsupp: &Partsupp, nation: &Nation) -> TypeQ11 {
    let mut n_h: HashMap<i32, Record<(i32,)>> = (0..nation.4)
        .into_iter()
        .filter(|&i| nation.1[i as usize] == VarChar::from_str("GERMANY").unwrap())
        .map(|i| (nation.0[i as usize], Record::new((nation.0[i as usize],))))
        .collect();
    let mut s_h: HashMap<i32, Bool> = (0..supplier.7)
        .into_iter()
        .filter(|&i| n_h.contains_key(&supplier.3[i as usize]))
        .map(|i| (supplier.0[i as usize], TRUE))
        .collect();
    let mut ps_t: Record<(OrderedFloat<f64>, HashMap<i32, OrderedFloat<f64>>)> = (0..partsupp.5)
        .into_iter()
        .filter(|&i| s_h.contains_key(&partsupp.1[i as usize]))
        .map(|i| {
            Record::new((
                partsupp.3[i as usize] * partsupp.2[i as usize] * OrderedFloat(0.0001f64),
                HashMap::from([(
                    partsupp.0[i as usize],
                    partsupp.3[i as usize] * partsupp.2[i as usize],
                )]),
            ))
        })
        .sum();
    let (ps_t_0, ps_t_1) = ps_t.decat();
    ps_t_1
        .into_iter()
        .filter(|&(ps_partkey, ps_supplycost)| ps_t_0 < ps_supplycost)
        .map(|(ps_partkey, ps_supplycost)| (Record::new((ps_partkey, ps_supplycost)), TRUE))
        .collect()
}

pub fn tpch_11_parallel(supplier: &Supplier, partsupp: &Partsupp, nation: &Nation) -> TypeQ11 {
    let mut n_h: HashMap<i32, Record<(i32,)>> = (0..nation.4)
        .into_par_iter()
        .filter(|&i| nation.1[i as usize] == VarChar::from_str("GERMANY").unwrap())
        .map(|i| (nation.0[i as usize], Record::new((nation.0[i as usize],))))
        .collect();
    let mut s_h: HashMap<i32, Bool> = (0..supplier.7)
        .into_par_iter()
        .filter(|&i| n_h.contains_key(&supplier.3[i as usize]))
        .map(|i| (supplier.0[i as usize], TRUE))
        .collect();
    let mut ps_t: Record<(OrderedFloat<f64>, HashMap<i32, OrderedFloat<f64>>)> = (0..partsupp.5)
        .into_par_iter()
        .filter(|&i| s_h.contains_key(&partsupp.1[i as usize]))
        .map(|i| {
            Record::new((
                partsupp.3[i as usize] * partsupp.2[i as usize] * OrderedFloat(0.0001f64),
                HashMap::from([(
                    partsupp.0[i as usize],
                    partsupp.3[i as usize] * partsupp.2[i as usize],
                )]),
            ))
        })
        .sum();
    let (ps_t_0, ps_t_1) = ps_t.decat();
    ps_t_1
        .into_par_iter()
        .filter(|&(ps_partkey, ps_supplycost)| ps_t_0 < ps_supplycost)
        .map(|(ps_partkey, ps_supplycost)| (Record::new((ps_partkey, ps_supplycost)), TRUE))
        .collect()
}
