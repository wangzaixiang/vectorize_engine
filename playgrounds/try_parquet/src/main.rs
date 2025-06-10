use std::path::Path;
use parquet::column::reader::ColumnReader::Int64ColumnReader;
use parquet::file::metadata::RowGroupMetaData;
use parquet::file::reader::{FileReader, SerializedFileReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let path = Path::new("/Volumes/wangzx-sandisk/workspaces/github.com//datafusion-duckdb-benchmark/clickbench/hits_multi/hits_0.parquet");

    let file = std::fs::File::open(path)?;

    let reader = SerializedFileReader::new(file)?;

    let metadata = reader.metadata();
    println!("{:?}", metadata.file_metadata());

    println!("num_row_groups:{}",reader.num_row_groups());
    let row_group_0 = reader.get_row_group(0)?;
    let row_group_1 = reader.get_row_group(0)?;
    println!("{:?}", row_group_0.metadata());

    let ir = reader.get_row_iter(None)?;
    for row in ir.take(2) {
        let row = row?;
        println!("{:?}", row);
    }


    // let x = row_group_0.get_row_iter(None)?;
    // // x.take(10).for_each(|r| println!("{:?}", r.unwrap()));
    // // println!("{}", x);
    //
    // let mut column_metadatas_0 = (0..row_group_0.num_columns()).map( |i| row_group_0.metadata().column(i).clone() ).collect::<Vec<_>>();
    // let column_metadatas_1 = (0..row_group_1.num_columns()).map( |i| row_group_1.metadata().column(i).clone() ).collect::<Vec<_>>();
    // column_metadatas_0.extend(column_metadatas_1);
    // column_metadatas_0.sort_by_key(|it| it.file_offset());
    //
    // // column_metadatas_0.iter().for_each(|it| println!("offset:{}, size: {}, file offset: {}", it.data_page_offset(), it.compressed_size(), it.file_offset()));

    Ok(())
}
