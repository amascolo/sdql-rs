use super::structs::{Customer, Lineitem, Orders};
use crate::utils::date_to_numeric;
use polars::prelude::*;
use std::error::Error;

pub fn read_q3_polars(
    customers_path: &str,
    orders_path: &str,
    lineitems_path: &str,
) -> Result<(Customer, Orders, Lineitem), Box<dyn Error>> {
    Ok((
        read_customers_polars(customers_path)?,
        read_orders_polars(orders_path)?,
        read_lineitems_polars(lineitems_path)?,
    ))
}

fn read_customers_polars(path: &str) -> Result<Customer, Box<dyn Error>> {
    let schema = Schema::from_iter([
        Field::new("custkey".into(), DataType::Int32),
        Field::new("name".into(), DataType::String),
        Field::new("address".into(), DataType::String),
        Field::new("nationkey".into(), DataType::Int32),
        Field::new("phone".into(), DataType::String),
        Field::new("acctbal".into(), DataType::Float64),
        Field::new("mktsegment".into(), DataType::String),
        Field::new("comment".into(), DataType::String),
    ]);

    let parse_options = CsvParseOptions::default().with_separator(b'|');
    let read_options = CsvReadOptions::default()
        .with_has_header(false)
        .with_parse_options(parse_options)
        .with_schema(Some(schema.into()));

    let df = read_options
        .try_into_reader_with_file_path(Some(path.into()))?
        .finish()?;

    let custkey = df
        .column("custkey")?
        .i32()?
        .into_iter()
        .map(|x| x.unwrap_or_default())
        .collect();
    let name: Vec<String> = df
        .column("name")?
        .str()?
        .into_iter()
        .map(|x| x.unwrap_or("").to_string())
        .collect();
    let address = df
        .column("address")?
        .str()?
        .into_iter()
        .map(|x| x.unwrap_or("").to_string())
        .collect();
    let nationkey = df
        .column("nationkey")?
        .i32()?
        .into_iter()
        .map(|x| x.unwrap_or_default())
        .collect();
    let phone = df
        .column("phone")?
        .str()?
        .into_iter()
        .map(|x| x.unwrap_or("").to_string())
        .collect();
    let acctbal = df
        .column("acctbal")?
        .f64()?
        .into_iter()
        .map(|x| x.unwrap_or_default())
        .collect();
    let mktsegment = df
        .column("mktsegment")?
        .str()?
        .into_iter()
        .map(|x| x.unwrap_or("").to_string())
        .collect();
    let comment = df
        .column("comment")?
        .str()?
        .into_iter()
        .map(|x| x.unwrap_or("").to_string())
        .collect();

    Ok(Customer {
        size: df.height(),
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

fn read_orders_polars(path: &str) -> Result<Orders, Box<dyn Error>> {
    let schema = Schema::from_iter([
        Field::new("orderkey".into(), DataType::Int32),
        Field::new("custkey".into(), DataType::Int32),
        Field::new("orderstatus".into(), DataType::String),
        Field::new("totalprice".into(), DataType::Float64),
        Field::new("orderdate".into(), DataType::String),
        Field::new("orderpriority".into(), DataType::String),
        Field::new("clerk".into(), DataType::String),
        Field::new("shippriority".into(), DataType::Int32),
        Field::new("comment".into(), DataType::String),
    ]);

    let parse_options = CsvParseOptions::default().with_separator(b'|');
    let read_options = CsvReadOptions::default()
        .with_has_header(false)
        .with_parse_options(parse_options)
        .with_schema(Some(schema.into()));

    let df = read_options
        .try_into_reader_with_file_path(Some(path.into()))?
        .finish()?;

    let orderkey = df
        .column("orderkey")?
        .i32()?
        .into_iter()
        .map(|x| x.unwrap_or_default())
        .collect();
    let custkey = df
        .column("custkey")?
        .i32()?
        .into_iter()
        .map(|x| x.unwrap_or_default())
        .collect();
    let orderstatus = df
        .column("orderstatus")?
        .str()?
        .into_iter()
        .map(|x| x.unwrap_or("").to_string())
        .collect();
    let totalprice = df
        .column("totalprice")?
        .f64()?
        .into_iter()
        .map(|x| x.unwrap_or_default())
        .collect();
    let orderdate = df
        .column("orderdate")?
        .str()?
        .into_iter()
        .map(|x| date_to_numeric(x.unwrap_or("")))
        .collect();
    let orderpriority = df
        .column("orderpriority")?
        .str()?
        .into_iter()
        .map(|x| x.unwrap_or("").to_string())
        .collect();
    let clerk = df
        .column("clerk")?
        .str()?
        .into_iter()
        .map(|x| x.unwrap_or("").to_string())
        .collect();
    let shippriority = df
        .column("shippriority")?
        .i32()?
        .into_iter()
        .map(|x| x.unwrap_or_default())
        .collect();
    let comment = df
        .column("comment")?
        .str()?
        .into_iter()
        .map(|x| x.unwrap_or("").to_string())
        .collect();

    // Return the populated Orders struct
    Ok(Orders {
        size: df.height(),
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

fn read_lineitems_polars(path: &str) -> Result<Lineitem, Box<dyn Error>> {
    // Define the schema for the lineitem table
    let schema = Schema::from_iter([
        Field::new("orderkey".into(), DataType::Int32),
        Field::new("partkey".into(), DataType::Int32),
        Field::new("suppkey".into(), DataType::Int32),
        Field::new("linenumber".into(), DataType::Int32),
        Field::new("quantity".into(), DataType::Float64),
        Field::new("extendedprice".into(), DataType::Float64),
        Field::new("discount".into(), DataType::Float64),
        Field::new("tax".into(), DataType::Float64),
        Field::new("returnflag".into(), DataType::String),
        Field::new("linestatus".into(), DataType::String),
        Field::new("shipdate".into(), DataType::String),
        Field::new("commitdate".into(), DataType::String),
        Field::new("receiptdate".into(), DataType::String),
        Field::new("shipinstruct".into(), DataType::String),
        Field::new("shipmode".into(), DataType::String),
        Field::new("comment".into(), DataType::String),
    ]);

    let parse_options = CsvParseOptions::default().with_separator(b'|');
    let read_options = CsvReadOptions::default()
        .with_has_header(false)
        .with_parse_options(parse_options)
        .with_schema(Some(schema.into()));

    let df = read_options
        .try_into_reader_with_file_path(Some(path.into()))?
        .finish()?;

    let orderkey = df
        .column("orderkey")?
        .i32()?
        .into_iter()
        .map(|x| x.unwrap_or_default())
        .collect();
    let partkey = df
        .column("partkey")?
        .i32()?
        .into_iter()
        .map(|x| x.unwrap_or_default())
        .collect();
    let suppkey = df
        .column("suppkey")?
        .i32()?
        .into_iter()
        .map(|x| x.unwrap_or_default())
        .collect();
    let linenumber = df
        .column("linenumber")?
        .i32()?
        .into_iter()
        .map(|x| x.unwrap_or_default())
        .collect();
    let quantity = df
        .column("quantity")?
        .f64()?
        .into_iter()
        .map(|x| x.unwrap_or_default())
        .collect();
    let extendedprice = df
        .column("extendedprice")?
        .f64()?
        .into_iter()
        .map(|x| x.unwrap_or_default())
        .collect();
    let discount = df
        .column("discount")?
        .f64()?
        .into_iter()
        .map(|x| x.unwrap_or_default())
        .collect();
    let tax = df
        .column("tax")?
        .f64()?
        .into_iter()
        .map(|x| x.unwrap_or_default())
        .collect();

    let returnflag = df
        .column("returnflag")?
        .str()?
        .into_iter()
        .map(|x| x.unwrap_or("").to_string())
        .collect();
    let linestatus = df
        .column("linestatus")?
        .str()?
        .into_iter()
        .map(|x| x.unwrap_or("").to_string())
        .collect();

    let shipdate = df
        .column("shipdate")?
        .str()?
        .into_iter()
        .map(|x| date_to_numeric(x.unwrap_or("")))
        .collect();
    let commitdate = df
        .column("commitdate")?
        .str()?
        .into_iter()
        .map(|x| date_to_numeric(x.unwrap_or("")))
        .collect();
    let receiptdate = df
        .column("receiptdate")?
        .str()?
        .into_iter()
        .map(|x| date_to_numeric(x.unwrap_or("")))
        .collect();

    let shipinstruct = df
        .column("shipinstruct")?
        .str()?
        .into_iter()
        .map(|x| x.unwrap_or("").to_string())
        .collect();
    let shipmode = df
        .column("shipmode")?
        .str()?
        .into_iter()
        .map(|x| x.unwrap_or("").to_string())
        .collect();
    let comment = df
        .column("comment")?
        .str()?
        .into_iter()
        .map(|x| x.unwrap_or("").to_string())
        .collect();

    Ok(Lineitem {
        size: df.height(),
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
