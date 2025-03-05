#[macro_export]
macro_rules! load {
    ($($field:ident: $ty:ty),*) => {
        #[allow(unused_assignments)]
        |path: &str| -> Result<( $(Vec<$ty>,)* usize ), Box<dyn std::error::Error>> {
            let mut reader = csv::ReaderBuilder::new()
                .has_headers(false)
                .delimiter(b'|')
                .from_path(path)?;

            $( let mut $field = Vec::new(); )*
            let mut size = 0;

            for result in reader.records() {
                let record = result?;
                let mut index = 0;
                $(
                    $field.push(record.get(index).unwrap().parse()?);
                    index += 1;
                )*
                size += 1;
            }

            Ok(( $( $field, )* size ))
        }
    };
}
