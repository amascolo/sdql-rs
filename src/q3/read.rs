use super::structs::{Customer, Lineitem, Orders};
use crate::utils::date_to_numeric;
use csv::ReaderBuilder;
use std::error::Error;

pub fn read_q3(
    customers_path: &str,
    orders_path: &str,
    lineitems_path: &str,
) -> Result<(Customer, Orders, Lineitem), Box<dyn Error>> {
    Ok((
        read_customers(customers_path)?,
        read_orders(orders_path)?,
        read_lineitems(lineitems_path)?,
    ))
}

fn read_customers(path: &str) -> Result<Customer, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(path)?;
    let mut custkey = Vec::new();
    let mut name = Vec::new();
    let mut address = Vec::new();
    let mut nationkey = Vec::new();
    let mut phone = Vec::new();
    let mut acctbal = Vec::new();
    let mut mktsegment = Vec::new();
    let mut comment = Vec::new();

    for result in reader.records() {
        let record = result?;
        custkey.push(record.get(0).unwrap().parse()?);
        name.push(record.get(1).unwrap().to_string());
        address.push(record.get(2).unwrap().to_string());
        nationkey.push(record.get(3).unwrap().parse()?);
        phone.push(record.get(4).unwrap().to_string());
        acctbal.push(record.get(5).unwrap().parse()?);
        mktsegment.push(record.get(6).unwrap().to_string());
        comment.push(record.get(7).unwrap().to_string());
    }
    Ok(Customer {
        size: custkey.len(),
        custkey,
        name,
        address,
        nationkey,
        phone,
        acctbal,
        mktsegment,
        comment,
    })
}

fn read_orders(path: &str) -> Result<Orders, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(path)?;
    let mut orderkey = Vec::new();
    let mut custkey = Vec::new();
    let mut orderstatus = Vec::new();
    let mut totalprice = Vec::new();
    let mut orderdate = Vec::new();
    let mut orderpriority = Vec::new();
    let mut clerk = Vec::new();
    let mut shippriority = Vec::new();
    let mut comment = Vec::new();

    for result in reader.records() {
        let record = result?;
        orderkey.push(record.get(0).unwrap().parse()?);
        custkey.push(record.get(1).unwrap().parse()?);
        orderstatus.push(record.get(2).unwrap().to_string());
        totalprice.push(record.get(3).unwrap().parse()?);
        orderdate.push(date_to_numeric(record.get(4).unwrap()));
        orderpriority.push(record.get(5).unwrap().to_string());
        clerk.push(record.get(6).unwrap().to_string());
        shippriority.push(record.get(7).unwrap().parse()?);
        comment.push(record.get(8).unwrap().to_string());
    }
    Ok(Orders {
        size: orderkey.len(),
        orderkey,
        custkey,
        orderstatus,
        totalprice,
        orderdate,
        orderpriority,
        clerk,
        shippriority,
        comment,
    })
}

fn read_lineitems(path: &str) -> Result<Lineitem, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(path)?;
    let mut orderkey = Vec::new();
    let mut partkey = Vec::new();
    let mut suppkey = Vec::new();
    let mut linenumber = Vec::new();
    let mut quantity = Vec::new();
    let mut extendedprice = Vec::new();
    let mut discount = Vec::new();
    let mut tax = Vec::new();
    let mut returnflag = Vec::new();
    let mut linestatus = Vec::new();
    let mut shipdate = Vec::new();
    let mut commitdate = Vec::new();
    let mut receiptdate = Vec::new();
    let mut shipinstruct = Vec::new();
    let mut shipmode = Vec::new();
    let mut comment = Vec::new();

    for result in reader.records() {
        let record = result?;
        orderkey.push(record.get(0).unwrap().parse()?);
        partkey.push(record.get(1).unwrap().parse()?);
        suppkey.push(record.get(2).unwrap().parse()?);
        linenumber.push(record.get(3).unwrap().parse()?);
        quantity.push(record.get(4).unwrap().parse()?);
        extendedprice.push(record.get(5).unwrap().parse()?);
        discount.push(record.get(6).unwrap().parse()?);
        tax.push(record.get(7).unwrap().parse()?);
        returnflag.push(record.get(8).unwrap().to_string());
        linestatus.push(record.get(9).unwrap().to_string());
        shipdate.push(date_to_numeric(record.get(10).unwrap()));
        commitdate.push(date_to_numeric(record.get(11).unwrap()));
        receiptdate.push(date_to_numeric(record.get(12).unwrap()));
        shipinstruct.push(record.get(13).unwrap().to_string());
        shipmode.push(record.get(14).unwrap().to_string());
        comment.push(record.get(15).unwrap().to_string());
    }
    Ok(Lineitem {
        size: orderkey.len(),
        orderkey,
        partkey,
        suppkey,
        linenumber,
        quantity,
        extendedprice,
        discount,
        tax,
        returnflag,
        linestatus,
        shipdate,
        commitdate,
        receiptdate,
        shipinstruct,
        shipmode,
        comment,
    })
}
