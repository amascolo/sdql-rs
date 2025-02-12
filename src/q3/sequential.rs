use super::read::{read_q3, Customer, Lineitem, Orders};
use hashbrown::HashMap;
use ordered_float::OrderedFloat;
use std::error::Error;

type TypeQ3 = HashMap<(i32, i32, i32, OrderedFloat<f64>), i32>;

pub fn q3() -> Result<TypeQ3, Box<dyn Error>> {
    let (customer, orders, lineitem) = read_q3(
        "datasets/tpch_datasets/SF_1/customer.tbl",
        "datasets/tpch_datasets/SF_1/orders.tbl",
        "datasets/tpch_datasets/SF_1/lineitem.tbl",
    )?;
    Ok(q3_query(&customer, &orders, &lineitem))
}

pub fn q3_query(customer: &Customer, orders: &Orders, lineitem: &Lineitem) -> TypeQ3 {
    let c_h: HashMap<_, _> = (0..customer.size)
        .filter(|&i| customer.mktsegment[i] == "BUILDING")
        .map(|i| (customer.custkey[i], customer.custkey[i]))
        .collect();

    let o_h: HashMap<_, _> = (0..orders.size)
        .filter(|&i| c_h.contains_key(&orders.custkey[i]))
        .filter(|&i| orders.orderdate[i] < 19950315)
        .map(|i| {
            (
                orders.orderkey[i],
                (orders.orderdate[i], orders.shippriority[i]),
            )
        })
        .collect();

    let l_h = (0..lineitem.size)
        .filter(|&i| lineitem.shipdate[i] > 19950315)
        .filter(|&i| o_h.contains_key(&lineitem.orderkey[i]))
        .fold(HashMap::new(), |mut acc, i| {
            *acc.entry((
                lineitem.orderkey[i],
                o_h[&lineitem.orderkey[i]].0,
                o_h[&lineitem.orderkey[i]].1,
            ))
            .or_default() += lineitem.extendedprice[i] * (1.0 - lineitem.discount[i]);
            acc
        });

    l_h.into_iter()
        .map(|(key, val)| ((key.0, key.1, key.2, OrderedFloat(val)), 1))
        .collect()
}
