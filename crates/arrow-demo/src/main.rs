use std::fs::File;
use std::sync::Arc;
use arrow::array::Array;

use arrow::csv;
use arrow::datatypes::{DataType, Field, Schema};
use arrow::util::pretty::print_batches;

fn main() {
    let schema = Schema::new(vec![
        Field::new("city", DataType::Utf8, false),
        Field::new("lat", DataType::Float64, false),
        Field::new("lng", DataType::Float64, false),
    ]);


    let file = File::open("./crates/arrow-demo/src/uk_cities.csv").unwrap();

    let mut csv =
        csv::Reader::new(file, Arc::new(schema), false, None, 1024, None, None, None);
    let batch = csv.next().unwrap().unwrap();
    for x in batch.columns() {
        println!("{}",x.data().len())
    }
    print_batches(&[batch]).unwrap();
}