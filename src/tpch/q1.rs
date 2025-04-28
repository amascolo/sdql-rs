#![allow(unused_mut, unused_variables)]

use super::types::*;
use rayon::prelude::*;
use sdql_runtime::*;

pub fn q1_query(lineitem: &Lineitem) -> TypeQ1 {
    let mut l_h: HashMap<
        Record<(VarChar<1>, VarChar<1>)>,
        Record<(
            OrderedFloat<f64>,
            OrderedFloat<f64>,
            OrderedFloat<f64>,
            OrderedFloat<f64>,
            i32,
        )>,
    > = (0..lineitem.16)
        .into_iter()
        .filter(|&i| lineitem.10[i as usize] <= date!(19980902))
        .fold(
            HashMap::default(),
            |mut acc: HashMap<
                Record<(VarChar<1>, VarChar<1>)>,
                Record<(
                    OrderedFloat<f64>,
                    OrderedFloat<f64>,
                    OrderedFloat<f64>,
                    OrderedFloat<f64>,
                    i32,
                )>,
            >,
             i| {
                acc[&Record::new((lineitem.8[i as usize], lineitem.9[i as usize]))] +=
                    Record::new((
                        lineitem.4[i as usize],
                        lineitem.5[i as usize],
                        lineitem.5[i as usize] * (OrderedFloat(1f64) - lineitem.6[i as usize]),
                        lineitem.5[i as usize]
                            * (OrderedFloat(1f64) - lineitem.6[i as usize])
                            * (OrderedFloat(1f64) + lineitem.7[i as usize]),
                        1i32,
                    ));
                acc
            },
        );
    l_h.into_iter().fold(
        HashMap::default(),
        |mut acc: HashMap<
            Record<(
                VarChar<1>,
                VarChar<1>,
                OrderedFloat<f64>,
                OrderedFloat<f64>,
                OrderedFloat<f64>,
                OrderedFloat<f64>,
                i32,
            )>,
            Bool,
        >,
         (k, v)| {
            acc[&Record::new((k.0, k.1, v.0, v.1, v.2, v.3, v.4))] += TRUE;
            acc
        },
    )
}

pub fn q1_query_rayon(lineitem: &Lineitem) -> TypeQ1 {
    let mut l_h: HashMap<
        Record<(VarChar<1>, VarChar<1>)>,
        Record<(
            OrderedFloat<f64>,
            OrderedFloat<f64>,
            OrderedFloat<f64>,
            OrderedFloat<f64>,
            i32,
        )>,
    > = (0..lineitem.16)
        .into_par_iter()
        .filter(|&i| lineitem.10[i as usize] <= date!(19980902))
        .fold(
            HashMap::default,
            |mut acc: HashMap<
                Record<(VarChar<1>, VarChar<1>)>,
                Record<(
                    OrderedFloat<f64>,
                    OrderedFloat<f64>,
                    OrderedFloat<f64>,
                    OrderedFloat<f64>,
                    i32,
                )>,
            >,
             i| {
                acc[&Record::new((lineitem.8[i as usize], lineitem.9[i as usize]))] +=
                    Record::new((
                        lineitem.4[i as usize],
                        lineitem.5[i as usize],
                        lineitem.5[i as usize] * (OrderedFloat(1f64) - lineitem.6[i as usize]),
                        lineitem.5[i as usize]
                            * (OrderedFloat(1f64) - lineitem.6[i as usize])
                            * (OrderedFloat(1f64) + lineitem.7[i as usize]),
                        1i32,
                    ));
                acc
            },
        )
        .sum();
    l_h.into_par_iter()
        .fold(
            HashMap::default,
            |mut acc: HashMap<
                Record<(
                    VarChar<1>,
                    VarChar<1>,
                    OrderedFloat<f64>,
                    OrderedFloat<f64>,
                    OrderedFloat<f64>,
                    OrderedFloat<f64>,
                    i32,
                )>,
                Bool,
            >,
             (k, v)| {
                acc[&Record::new((k.0, k.1, v.0, v.1, v.2, v.3, v.4))] += TRUE;
                acc
            },
        )
        .sum()
}
