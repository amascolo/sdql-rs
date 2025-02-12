use super::types::{Customer, Lineitem, Orders};
use crate::utils::date_to_numeric;
use polars::prelude::*;
use polars_arrow::array::{Float64Array, Int32Array};
use std::error::Error;
use std::mem;

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

fn i32arrow_array_into_vec(arr: Int32Array) -> Vec<i32> {
    let length = arr.len();
    let (_dtype, buffer, _validity) = arr.into_inner();
    let ptr = buffer.as_ptr() as *mut i32;
    let capacity = buffer.len() / size_of::<i32>();
    // prevent dropping original array and deallocating buffer
    mem::forget(buffer);
    unsafe { Vec::from_raw_parts(ptr, length, capacity) }
}

fn f64arrow_array_into_vec(arr: Float64Array) -> Vec<f64> {
    let length = arr.len();
    let (_dtype, buffer, _validity) = arr.into_inner();
    let ptr = buffer.as_ptr() as *mut f64;
    let capacity = buffer.len() / size_of::<f64>();
    // prevent dropping original array and deallocating buffer
    mem::forget(buffer);
    unsafe { Vec::from_raw_parts(ptr, length, capacity) }
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
        .with_schema(Some(schema.into()))
        .with_rechunk(true);

    let df = read_options
        .try_into_reader_with_file_path(Some(path.into()))?
        .finish()?;

    let custkey = i32arrow_array_into_vec(
        df.column("custkey")?
            .i32()?
            .downcast_iter()
            .next()
            .unwrap()
            .clone(),
    );
    let name: Vec<String> = df
        .column("name")?
        .str()?
        .into_iter()
        .map(|x| x.unwrap_or("").to_string())
        .collect();
    let address: Vec<String> = df
        .column("address")?
        .str()?
        .into_iter()
        .map(|x| x.unwrap_or("").to_string())
        .collect();
    let nationkey = i32arrow_array_into_vec(
        df.column("nationkey")?
            .i32()?
            .downcast_iter()
            .next()
            .unwrap()
            .clone(),
    );
    let phone: Vec<String> = df
        .column("phone")?
        .str()?
        .into_iter()
        .map(|x| x.unwrap_or("").to_string())
        .collect();
    let acctbal = f64arrow_array_into_vec(
        df.column("acctbal")?
            .f64()?
            .downcast_iter()
            .next()
            .unwrap()
            .clone(),
    );
    let mktsegment: Vec<String> = df
        .column("mktsegment")?
        .str()?
        .into_iter()
        .map(|x| x.unwrap_or("").to_string())
        .collect();
    let comment: Vec<String> = df
        .column("comment")?
        .str()?
        .into_iter()
        .map(|x| x.unwrap_or("").to_string())
        .collect();

    Ok((
        custkey,
        name,
        address,
        nationkey,
        phone,
        acctbal,
        mktsegment,
        comment,
        df.height(),
    ))
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
        .with_schema(Some(schema.into()))
        .with_rechunk(true);

    let df = read_options
        .try_into_reader_with_file_path(Some(path.into()))?
        .finish()?;

    let orderkey = i32arrow_array_into_vec(
        df.column("orderkey")?
            .i32()?
            .downcast_iter()
            .next()
            .unwrap()
            .clone(),
    );
    let custkey = i32arrow_array_into_vec(
        df.column("custkey")?
            .i32()?
            .downcast_iter()
            .next()
            .unwrap()
            .clone(),
    );
    let orderstatus: Vec<String> = df
        .column("orderstatus")?
        .str()?
        .into_iter()
        .map(|x| x.unwrap_or("").to_string())
        .collect();
    let totalprice = f64arrow_array_into_vec(
        df.column("totalprice")?
            .f64()?
            .downcast_iter()
            .next()
            .unwrap()
            .clone(),
    );
    let orderdate = df
        .column("orderdate")?
        .str()?
        .into_iter()
        .map(|x| date_to_numeric(x.unwrap_or("")))
        .collect();
    let orderpriority: Vec<String> = df
        .column("orderpriority")?
        .str()?
        .into_iter()
        .map(|x| x.unwrap_or("").to_string())
        .collect();
    let clerk: Vec<String> = df
        .column("clerk")?
        .str()?
        .into_iter()
        .map(|x| x.unwrap_or("").to_string())
        .collect();
    let shippriority = i32arrow_array_into_vec(
        df.column("shippriority")?
            .i32()?
            .downcast_iter()
            .next()
            .unwrap()
            .clone(),
    );
    let comment: Vec<String> = df
        .column("comment")?
        .str()?
        .into_iter()
        .map(|x| x.unwrap_or("").to_string())
        .collect();

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
        df.height(),
    ))
}
fn read_lineitems_polars(path: &str) -> Result<Lineitem, Box<dyn Error>> {
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
        .with_schema(Some(schema.into()))
        .with_rechunk(true);

    let df = read_options
        .try_into_reader_with_file_path(Some(path.into()))?
        .finish()?;

    let orderkey = i32arrow_array_into_vec(
        df.column("orderkey")?
            .i32()?
            .downcast_iter()
            .next()
            .unwrap()
            .clone(),
    );
    let partkey = i32arrow_array_into_vec(
        df.column("partkey")?
            .i32()?
            .downcast_iter()
            .next()
            .unwrap()
            .clone(),
    );
    let suppkey = i32arrow_array_into_vec(
        df.column("suppkey")?
            .i32()?
            .downcast_iter()
            .next()
            .unwrap()
            .clone(),
    );
    let linenumber = i32arrow_array_into_vec(
        df.column("linenumber")?
            .i32()?
            .downcast_iter()
            .next()
            .unwrap()
            .clone(),
    );
    let quantity = f64arrow_array_into_vec(
        df.column("quantity")?
            .f64()?
            .downcast_iter()
            .next()
            .unwrap()
            .clone(),
    );
    let extendedprice = f64arrow_array_into_vec(
        df.column("extendedprice")?
            .f64()?
            .downcast_iter()
            .next()
            .unwrap()
            .clone(),
    );
    let discount = f64arrow_array_into_vec(
        df.column("discount")?
            .f64()?
            .downcast_iter()
            .next()
            .unwrap()
            .clone(),
    );
    let tax = f64arrow_array_into_vec(
        df.column("tax")?
            .f64()?
            .downcast_iter()
            .next()
            .unwrap()
            .clone(),
    );
    let returnflag: Vec<String> = df
        .column("returnflag")?
        .str()?
        .into_iter()
        .map(|x| x.unwrap_or("").to_string())
        .collect();
    let linestatus: Vec<String> = df
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
    let shipinstruct: Vec<String> = df
        .column("shipinstruct")?
        .str()?
        .into_iter()
        .map(|x| x.unwrap_or("").to_string())
        .collect();
    let shipmode: Vec<String> = df
        .column("shipmode")?
        .str()?
        .into_iter()
        .map(|x| x.unwrap_or("").to_string())
        .collect();
    let comment: Vec<String> = df
        .column("comment")?
        .str()?
        .into_iter()
        .map(|x| x.unwrap_or("").to_string())
        .collect();

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
        df.height(),
    ))
}
