use super::read::read_q3;
use super::types::{Customer, Lineitem, Orders};
use crate::utils::print_date;
use hashbrown::HashMap;
use ordered_float::OrderedFloat;
use std::error::Error;

type TypeQ3 = HashMap<(i32, i32, i32, OrderedFloat<f64>), i32>;

pub fn print_q3_result(result: TypeQ3) {
    for (key, val) in result.iter() {
        println!(
            "<{}, {}, {}, {}>:{}",
            key.0,
            print_date(key.1),
            key.2,
            key.3,
            val
        );
    }
}

pub fn q3() -> Result<TypeQ3, Box<dyn Error>> {
    let (customer, orders, lineitem) = read_q3(
        "datasets/tpch_datasets/SF_1/customer.tbl",
        "datasets/tpch_datasets/SF_1/orders.tbl",
        "datasets/tpch_datasets/SF_1/lineitem.tbl",
    )?;
    Ok(q3_query(&customer, &orders, &lineitem))
}

pub fn q3_query(customer: &Customer, orders: &Orders, lineitem: &Lineitem) -> TypeQ3 {
    let c_h: HashMap<_, _> = (0../* size */ customer.8)
        .filter(|&i| /* mktsegment */ customer.6[i] == "BUILDING")
        .map(|i| {
            (
                /* custkey */ customer.0[i],
                /* custkey */ customer.0[i],
            )
        })
        .collect();

    let o_h: HashMap<_, _> = (0../* size */ orders.9)
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
        .filter(|&i| /* shipdate */ lineitem.10[i] > 19950315)
        .filter(|&i| o_h.contains_key(&/* orderkey */ lineitem.0[i]))
        .fold(HashMap::new(), |mut acc, i| {
            *acc.entry((
                /* orderkey */ lineitem.0[i],
                o_h[&/* orderkey */ lineitem.0[i]].0,
                o_h[&/* orderkey */ lineitem.0[i]].1,
            ))
            .or_default() += /* extendedprice */ lineitem.5[i] * (1.0 - /* discount */ lineitem.6[i]);
            acc
        });

    l_h.into_iter()
        .map(|(key, val)| ((key.0, key.1, key.2, OrderedFloat(val)), 1))
        .collect()
}
