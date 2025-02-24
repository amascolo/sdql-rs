use crate::tpch::read::{read_customers, read_lineitems, read_orders};
use crate::tpch::types::{Customer, Lineitem, Orders};
use hashbrown::HashMap;
use ordered_float::OrderedFloat;
use rayon::prelude::*;
use std::error::Error;

type TypeQ3 = HashMap<(i32, i32, i32, OrderedFloat<f64>), i32>;

pub fn q3_rayon() -> Result<TypeQ3, Box<dyn Error>> {
    let customer = read_customers("datasets/tpch_datasets/SF_1/customer.tbl")?;
    let orders = read_orders("datasets/tpch_datasets/SF_1/orders.tbl")?;
    let lineitem = read_lineitems("datasets/tpch_datasets/SF_1/lineitem.tbl")?;
    Ok(q3_query_rayon(&customer, &orders, &lineitem))
}

pub fn q3_query_rayon(customer: &Customer, orders: &Orders, lineitem: &Lineitem) -> TypeQ3 {
    let c_h: HashMap<_, _> = (0../* size */ customer.8)
        .into_par_iter()
        .filter(|&i| /* mktsegment */ customer.6[i] == "BUILDING")
        .map(|i| {
            (
                /* custkey */ customer.0[i],
                /* custkey */ customer.0[i],
            )
        })
        .collect();

    let o_h: HashMap<_, _> = (0../* size */ orders.9)
        .into_par_iter()
        .filter(|&i| c_h.contains_key(&/* custkey */ orders.1[i]))
        .filter(|&i| /* orderdate */ orders.4[i] < 19950315)
        .map(|i| {
            (
                /* orderkey */ orders.0[i],
                (
                    /* orderdate */ orders.4[i],
                    /* shippriority */ orders.7[i],
                ),
            )
        })
        .collect();

    let l_h = (0../* size */ lineitem.16)
        .into_par_iter()
        .filter(|&i| /* shipdate */ lineitem.10[i] > 19950315)
        .filter(|&i| o_h.contains_key(&/* orderkey */ lineitem.0[i]))
        .fold(
            HashMap::new,
            |mut acc, i| {
                *acc.entry((
                    /* orderkey */ lineitem.0[i],
                    o_h[&/* orderkey */ lineitem.0[i]].0,
                    o_h[&/* orderkey */ lineitem.0[i]].1,
                ))
                .or_default() += /* extendedprice */ lineitem.5[i] * (1.0 - /* discount */ lineitem.6[i]);
                acc
            },
        )
        .reduce(
            HashMap::new,
            |mut acc, partial| {
                for (key, value) in partial {
                    *acc.entry(key).or_default() += value;
                }
                acc
            },
        );

    l_h.into_par_iter()
        .map(|(key, val)| ((key.0, key.1, key.2, OrderedFloat(val)), 1))
        .collect()
}
