use crate::runtime::Date;

macro_rules! read_fn {
    ($name:ident, $(($index:expr, $field:ident, $ty:ty)),*) => {
        pub fn $name(path: &str) -> Result<( $(Vec<$ty>,)* usize ), Box<dyn std::error::Error>> {
            let mut reader = csv::ReaderBuilder::new()
                .has_headers(false)
                .delimiter(b'|')
                .from_path(path)?;

            $( let mut $field = Vec::new(); )*
            let mut size = 0;

            for result in reader.records() {
                let record = result?;
                $(
                    $field.push(record.get($index).unwrap().parse()?);
                )*
                size += 1;
            }

            Ok(( $( $field, )* size ))
        }
    };
}

read_fn!(
    read_customers,
    (0, custkey, i32),
    (1, name, String),
    (2, address, String),
    (3, nationkey, i32),
    (4, phone, String),
    (5, acctbal, f64),
    (6, mktsegment, String),
    (7, comment, String)
);

read_fn!(
    read_orders,
    (0, orderkey, i32),
    (1, custkey, i32),
    (2, orderstatus, String),
    (3, totalprice, f64),
    (4, orderdate, Date),
    (5, orderpriority, String),
    (6, clerk, String),
    (7, shippriority, i32),
    (8, comment, String)
);

read_fn!(
    read_lineitems,
    (0, orderkey, i32),
    (1, partkey, i32),
    (2, suppkey, i32),
    (3, linenumber, i32),
    (4, quantity, f64),
    (5, extendedprice, f64),
    (6, discount, f64),
    (7, tax, f64),
    (8, returnflag, String),
    (9, linestatus, String),
    (10, shipdate, Date),
    (11, commitdate, Date),
    (12, receiptdate, Date),
    (13, shipinstruct, String),
    (14, shipmode, String),
    (15, comment, String)
);
