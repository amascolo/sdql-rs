use super::types::{Customer, Lineitem, Orders};
use crate::runtime::Date;
use csv::ReaderBuilder;
use std::error::Error;
use time::format_description::well_known::Iso8601;

pub fn read_customers(path: &str) -> Result<Customer, Box<dyn Error>> {
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
    let mut size = 0;

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
        size += 1;
    }
    Ok((
        custkey, name, address, nationkey, phone, acctbal, mktsegment, comment, size,
    ))
}

pub fn read_orders(path: &str) -> Result<Orders, Box<dyn Error>> {
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
    let mut size = 0;

    for result in reader.records() {
        let record = result?;
        orderkey.push(record.get(0).unwrap().parse()?);
        custkey.push(record.get(1).unwrap().parse()?);
        orderstatus.push(record.get(2).unwrap().to_string());
        totalprice.push(record.get(3).unwrap().parse()?);
        orderdate.push(Date::new(
            time::Date::parse(record.get(4).unwrap(), &Iso8601::DEFAULT).unwrap(),
        ));
        orderpriority.push(record.get(5).unwrap().to_string());
        clerk.push(record.get(6).unwrap().to_string());
        shippriority.push(record.get(7).unwrap().parse()?);
        comment.push(record.get(8).unwrap().to_string());
        size += 1;
    }
    Ok((
        orderkey,
        custkey,
        orderstatus,
        totalprice,
        orderdate,
        orderpriority,
        clerk,
        shippriority,
        comment,
        size,
    ))
}

pub fn read_lineitems(path: &str) -> Result<Lineitem, Box<dyn Error>> {
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
    let mut size = 0;

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
        shipdate.push(Date::new(
            time::Date::parse(record.get(10).unwrap(), &Iso8601::DEFAULT).unwrap(),
        ));
        commitdate.push(Date::new(
            time::Date::parse(record.get(11).unwrap(), &Iso8601::DEFAULT).unwrap(),
        ));
        receiptdate.push(Date::new(
            time::Date::parse(record.get(12).unwrap(), &Iso8601::DEFAULT).unwrap(),
        ));
        shipinstruct.push(record.get(13).unwrap().to_string());
        shipmode.push(record.get(14).unwrap().to_string());
        comment.push(record.get(15).unwrap().to_string());
        size += 1;
    }
    Ok((
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
        size,
    ))
}
