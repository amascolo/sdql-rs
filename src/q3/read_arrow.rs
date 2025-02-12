use super::structs::{Customer, Lineitem, Orders};
use crate::utils::date_to_numeric;
use arrow::array::{Float64Builder, Int32Builder, StringBuilder};
use csv::ReaderBuilder;
use std::error::Error;

pub fn read_q3_arrow(
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
    let mut custkey_builder = Int32Builder::new();
    let mut name_builder = StringBuilder::new();
    let mut address_builder = StringBuilder::new();
    let mut nationkey_builder = Int32Builder::new();
    let mut phone_builder = StringBuilder::new();
    let mut acctbal_builder = Float64Builder::new();
    let mut mktsegment_builder = StringBuilder::new();
    let mut comment_builder = StringBuilder::new();
    for result in reader.records() {
        let record = result?;
        custkey_builder.append_value(record.get(0).unwrap().parse::<i32>()?);
        name_builder.append_value(record.get(1).unwrap());
        address_builder.append_value(record.get(2).unwrap());
        nationkey_builder.append_value(record.get(3).unwrap().parse::<i32>()?);
        phone_builder.append_value(record.get(4).unwrap());
        acctbal_builder.append_value(record.get(5).unwrap().parse::<f64>()?);
        mktsegment_builder.append_value(record.get(6).unwrap());
        comment_builder.append_value(record.get(7).unwrap());
    }
    let custkey_array = custkey_builder.finish();
    let name_array = name_builder.finish();
    let address_array = address_builder.finish();
    let nationkey_array = nationkey_builder.finish();
    let phone_array = phone_builder.finish();
    let acctbal_array = acctbal_builder.finish();
    let mktsegment_array = mktsegment_builder.finish();
    let comment_array = comment_builder.finish();
    let size = custkey_array.len();
    let mut custkey = Vec::with_capacity(size);
    for i in 0..size {
        custkey.push(custkey_array.value(i));
    }
    let mut name = Vec::with_capacity(size);
    for i in 0..size {
        name.push(name_array.value(i).to_string());
    }
    let mut address = Vec::with_capacity(size);
    for i in 0..size {
        address.push(address_array.value(i).to_string());
    }
    let mut nationkey = Vec::with_capacity(size);
    for i in 0..size {
        nationkey.push(nationkey_array.value(i));
    }
    let mut phone = Vec::with_capacity(size);
    for i in 0..size {
        phone.push(phone_array.value(i).to_string());
    }
    let mut acctbal = Vec::with_capacity(size);
    for i in 0..size {
        acctbal.push(acctbal_array.value(i));
    }
    let mut mktsegment = Vec::with_capacity(size);
    for i in 0..size {
        mktsegment.push(mktsegment_array.value(i).to_string());
    }
    let mut comment = Vec::with_capacity(size);
    for i in 0..size {
        comment.push(comment_array.value(i).to_string());
    }
    Ok(Customer {
        size,
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

pub fn read_orders(path: &str) -> Result<Orders, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(path)?;
    let mut orderkey_builder = Int32Builder::new();
    let mut custkey_builder = Int32Builder::new();
    let mut orderstatus_builder = StringBuilder::new();
    let mut totalprice_builder = Float64Builder::new();
    let mut orderdate_builder = Int32Builder::new();
    let mut orderpriority_builder = StringBuilder::new();
    let mut clerk_builder = StringBuilder::new();
    let mut shippriority_builder = Int32Builder::new();
    let mut comment_builder = StringBuilder::new();
    for result in reader.records() {
        let record = result?;
        orderkey_builder.append_value(record.get(0).unwrap().parse::<i32>()?);
        custkey_builder.append_value(record.get(1).unwrap().parse::<i32>()?);
        orderstatus_builder.append_value(record.get(2).unwrap());
        totalprice_builder.append_value(record.get(3).unwrap().parse::<f64>()?);
        orderdate_builder.append_value(date_to_numeric(record.get(4).unwrap()));
        orderpriority_builder.append_value(record.get(5).unwrap());
        clerk_builder.append_value(record.get(6).unwrap());
        shippriority_builder.append_value(record.get(7).unwrap().parse::<i32>()?);
        comment_builder.append_value(record.get(8).unwrap());
    }
    let orderkey_array = orderkey_builder.finish();
    let custkey_array = custkey_builder.finish();
    let orderstatus_array = orderstatus_builder.finish();
    let totalprice_array = totalprice_builder.finish();
    let orderdate_array = orderdate_builder.finish();
    let orderpriority_array = orderpriority_builder.finish();
    let clerk_array = clerk_builder.finish();
    let shippriority_array = shippriority_builder.finish();
    let comment_array = comment_builder.finish();

    let size = orderkey_array.len();
    let mut orderkey = Vec::with_capacity(size);
    for i in 0..size {
        orderkey.push(orderkey_array.value(i));
    }
    let mut custkey = Vec::with_capacity(size);
    for i in 0..size {
        custkey.push(custkey_array.value(i));
    }
    let mut orderstatus = Vec::with_capacity(size);
    for i in 0..size {
        orderstatus.push(orderstatus_array.value(i).to_string());
    }
    let mut totalprice = Vec::with_capacity(size);
    for i in 0..size {
        totalprice.push(totalprice_array.value(i));
    }
    let mut orderdate = Vec::with_capacity(size);
    for i in 0..size {
        orderdate.push(orderdate_array.value(i));
    }
    let mut orderpriority = Vec::with_capacity(size);
    for i in 0..size {
        orderpriority.push(orderpriority_array.value(i).to_string());
    }
    let mut clerk = Vec::with_capacity(size);
    for i in 0..size {
        clerk.push(clerk_array.value(i).to_string());
    }
    let mut shippriority = Vec::with_capacity(size);
    for i in 0..size {
        shippriority.push(shippriority_array.value(i));
    }
    let mut comment = Vec::with_capacity(size);
    for i in 0..size {
        comment.push(comment_array.value(i).to_string());
    }
    Ok(Orders {
        size,
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

pub fn read_lineitems(path: &str) -> Result<Lineitem, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(path)?;
    let mut orderkey_builder = Int32Builder::new();
    let mut partkey_builder = Int32Builder::new();
    let mut suppkey_builder = Int32Builder::new();
    let mut linenumber_builder = Int32Builder::new();
    let mut quantity_builder = Float64Builder::new();
    let mut extendedprice_builder = Float64Builder::new();
    let mut discount_builder = Float64Builder::new();
    let mut tax_builder = Float64Builder::new();
    let mut returnflag_builder = StringBuilder::new();
    let mut linestatus_builder = StringBuilder::new();
    let mut shipdate_builder = Int32Builder::new();
    let mut commitdate_builder = Int32Builder::new();
    let mut receiptdate_builder = Int32Builder::new();
    let mut shipinstruct_builder = StringBuilder::new();
    let mut shipmode_builder = StringBuilder::new();
    let mut comment_builder = StringBuilder::new();
    for result in reader.records() {
        let record = result?;
        orderkey_builder.append_value(record.get(0).unwrap().parse::<i32>()?);
        partkey_builder.append_value(record.get(1).unwrap().parse::<i32>()?);
        suppkey_builder.append_value(record.get(2).unwrap().parse::<i32>()?);
        linenumber_builder.append_value(record.get(3).unwrap().parse::<i32>()?);
        quantity_builder.append_value(record.get(4).unwrap().parse::<f64>()?);
        extendedprice_builder.append_value(record.get(5).unwrap().parse::<f64>()?);
        discount_builder.append_value(record.get(6).unwrap().parse::<f64>()?);
        tax_builder.append_value(record.get(7).unwrap().parse::<f64>()?);
        returnflag_builder.append_value(record.get(8).unwrap());
        linestatus_builder.append_value(record.get(9).unwrap());
        shipdate_builder.append_value(date_to_numeric(record.get(10).unwrap()));
        commitdate_builder.append_value(date_to_numeric(record.get(11).unwrap()));
        receiptdate_builder.append_value(date_to_numeric(record.get(12).unwrap()));
        shipinstruct_builder.append_value(record.get(13).unwrap());
        shipmode_builder.append_value(record.get(14).unwrap());
        comment_builder.append_value(record.get(15).unwrap());
    }
    let orderkey_array = orderkey_builder.finish();
    let partkey_array = partkey_builder.finish();
    let suppkey_array = suppkey_builder.finish();
    let linenumber_array = linenumber_builder.finish();
    let quantity_array = quantity_builder.finish();
    let extendedprice_array = extendedprice_builder.finish();
    let discount_array = discount_builder.finish();
    let tax_array = tax_builder.finish();
    let returnflag_array = returnflag_builder.finish();
    let linestatus_array = linestatus_builder.finish();
    let shipdate_array = shipdate_builder.finish();
    let commitdate_array = commitdate_builder.finish();
    let receiptdate_array = receiptdate_builder.finish();
    let shipinstruct_array = shipinstruct_builder.finish();
    let shipmode_array = shipmode_builder.finish();
    let comment_array = comment_builder.finish();
    let size = orderkey_array.len();
    let mut orderkey = Vec::with_capacity(size);
    for i in 0..size {
        orderkey.push(orderkey_array.value(i));
    }
    let mut partkey = Vec::with_capacity(size);
    for i in 0..size {
        partkey.push(partkey_array.value(i));
    }
    let mut suppkey = Vec::with_capacity(size);
    for i in 0..size {
        suppkey.push(suppkey_array.value(i));
    }
    let mut linenumber = Vec::with_capacity(size);
    for i in 0..size {
        linenumber.push(linenumber_array.value(i));
    }
    let mut quantity = Vec::with_capacity(size);
    for i in 0..size {
        quantity.push(quantity_array.value(i));
    }
    let mut extendedprice = Vec::with_capacity(size);
    for i in 0..size {
        extendedprice.push(extendedprice_array.value(i));
    }
    let mut discount = Vec::with_capacity(size);
    for i in 0..size {
        discount.push(discount_array.value(i));
    }
    let mut tax = Vec::with_capacity(size);
    for i in 0..size {
        tax.push(tax_array.value(i));
    }
    let mut returnflag = Vec::with_capacity(size);
    for i in 0..size {
        returnflag.push(returnflag_array.value(i).to_string());
    }
    let mut linestatus = Vec::with_capacity(size);
    for i in 0..size {
        linestatus.push(linestatus_array.value(i).to_string());
    }
    let mut shipdate = Vec::with_capacity(size);
    for i in 0..size {
        shipdate.push(shipdate_array.value(i));
    }
    let mut commitdate = Vec::with_capacity(size);
    for i in 0..size {
        commitdate.push(commitdate_array.value(i));
    }
    let mut receiptdate = Vec::with_capacity(size);
    for i in 0..size {
        receiptdate.push(receiptdate_array.value(i));
    }
    let mut shipinstruct = Vec::with_capacity(size);
    for i in 0..size {
        shipinstruct.push(shipinstruct_array.value(i).to_string());
    }
    let mut shipmode = Vec::with_capacity(size);
    for i in 0..size {
        shipmode.push(shipmode_array.value(i).to_string());
    }
    let mut comment = Vec::with_capacity(size);
    for i in 0..size {
        comment.push(comment_array.value(i).to_string());
    }
    Ok(Lineitem {
        size,
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
