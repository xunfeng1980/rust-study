use arrow_array::{ArrayRef, Float32Array, Int32Array, RecordBatch, RecordBatchReader};
use std::sync::Arc;
use arrow::datatypes::{DataType, Field, Schema};
use lance::dataset;
use lance::dataset::*;

fn main() {

    // scanner.try_into_stream().await.unwrap();
    // let mut write_params = WriteParams::default();
    // let mut reader: Box<dyn RecordBatchReader<Item=(RecordBatch)>> = Box::new(batch);
    // Dataset::write(&mut reader, test_uri, Some(write_params)).await?;
}
