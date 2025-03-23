use super::_19950315;
use crate::tpch::read::{read_customers, read_lineitems, read_orders};
use crate::tpch::types::{Customer, Lineitem, Orders, TypeQ3};
use rayon::prelude::*;
use sdql_runtime::{Date, HashMap, OrderedFloat, Record, VarChar, TRUE};
use std::error::Error;

pub fn q3_rayon(sf: &str) -> Result<TypeQ3, Box<dyn Error>> {
    let customer = read_customers()(&format!("datasets/tpch_datasets/SF_{sf}/customer.tbl"))?;
    let orders = read_orders()(&format!("datasets/tpch_datasets/SF_{sf}/orders.tbl"))?;
    let lineitem = read_lineitems()(&format!("datasets/tpch_datasets/SF_{sf}/lineitem.tbl"))?;
    Ok(q3_query_rayon(&customer, &orders, &lineitem))
}

pub fn q3_query_rayon(customer: &Customer, orders: &Orders, lineitem: &Lineitem) -> TypeQ3 {
    let c_h: HashMap<i32, i32> = (0../* size */ customer.8)
        .into_par_iter()
        .filter(|&i| /* mktsegment */ customer.6[i] == VarChar::from("BUILDING").unwrap())
        .fold(HashMap::new, |mut acc, i| {
            acc[/* custkey */ &customer.0[i]] += /* custkey */ customer.0[i];
            acc
        })
        .reduce(HashMap::new, HashMap::sum);
    // TODO this is way faster (for parallel, not for sequential) - we can do it since key is unique
    // .map(|i| {
    //     (
    //         /* custkey */ customer.0[i],
    //         /* custkey */ customer.0[i],
    //     )
    // })
    //     .collect();

    let o_h: HashMap<i32, Record<(Date, i32)>> = (0../* size */ orders.9)
        .into_par_iter()
        .filter(|&i| c_h.contains_key(&/* custkey */ orders.1[i]))
        .filter(|&i| /* orderdate */ orders.4[i] < _19950315)
        .fold(HashMap::new, |mut acc, i| {
            acc[&/* orderkey */ orders.0[i]] += Record::new((
                /* orderdate */ orders.4[i],
                /* shippriority */ orders.7[i],
            ));
            acc
        })
        .reduce(HashMap::new, HashMap::sum);
    // TODO this is way faster (for parallel, not for sequential) - we can do it since key is unique
    // .map(|i| {
    //     (
    //         /* orderkey */ orders.0[i],
    //         Record::new((
    //             /* orderdate */ orders.4[i],
    //             /* shippriority */ orders.7[i],
    //         )),
    //     )
    // })
    // .collect();

    let l_h: HashMap<Record<(i32, Date, i32)>, OrderedFloat<f64>> = (0../* size */ lineitem.16)
        .into_par_iter()
        .filter(|&i| /* shipdate */ lineitem.10[i] > _19950315)
        .filter(|&i| o_h.contains_key(&/* orderkey */ lineitem.0[i]))
        .fold(HashMap::new, |mut acc, i| {
            acc[&Record::new((
                    /* orderkey */ lineitem.0[i],
                    o_h[&/* orderkey */ lineitem.0[i]].0,
                    o_h[&/* orderkey */ lineitem.0[i]].1,
                ))] += /* extendedprice */ lineitem.5[i] * (OrderedFloat(1.0) - /* discount */ lineitem.6[i]);
            acc
        })
        .reduce(HashMap::new, HashMap::sum);
    // TODO this is slower (benched on Q3 SF 1)
    // .reduce_with(HashMap::sum)
    // .unwrap_or_default();

    l_h.into_par_iter()
        .map(|(key, val)| (Record::new((key.0, key.1, key.2, val)), TRUE))
        .collect()
}
