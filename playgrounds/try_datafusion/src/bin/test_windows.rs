use std::sync::Arc;
use datafusion::arrow::array::{Date32Array, Float32Array, Float64Array, Int32Array, Int64Array, RecordBatch, UInt32Array, UInt64Array};
use datafusion::arrow::datatypes::{DataType, Field, Schema};
use datafusion::error::Result;
use datafusion::prelude::{ParquetReadOptions, SessionConfig, SessionContext};
use datafusion_datasource::memory::MemorySourceConfig;
use datafusion_datasource::source::DataSourceExec;
use rand::RngCore;

#[tokio::main]
async fn main() -> Result<()> {
    let mut ctx = SessionContext::new();
    _ = prepare(&mut ctx).await?;

    let df2 = ctx.sql("select *, \n\
    sum(amount) over (partition by product_id) as \"product_amounts\", -- WindowAggExec \n\
    sum(amount) over (partition by product_id order by order_date rows 1 preceding) as \"amounts1\", -- BoundedWindowAggExec \n\
    rank() over (partition by product_id order by order_date) as rank1, -- BoundedWindowAggExec \n\
    rank() over (partition by product_id order by order_date desc) as rank2 -- BoundedWindowAggExec \n\
      from t1").await?;

    df2.clone().explain(false, true)?.show().await?;

    _ = df2.show().await?;

    Ok(())
}

async fn prepare(ctx: &mut SessionContext) -> Result<()> {
    // build a memory dataframe with columns: product_id, order_date, quantity, amount
    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::UInt64, false),
        Field::new("product_id", DataType::UInt32, false),
        Field::new("order_date", DataType::Date32, false),
        // Field::new("price", DataType::Float64, false),
        Field::new("quantity", DataType::UInt32, false),
        Field::new("amount", DataType::Float64, false),
    ]));

    let mut rng = rand::thread_rng();

    let range = 0..30;
    let id_array: Vec<u64> = range.clone().map(|i| i).collect();
    let product_id_array: Vec<u32> = range.clone().map( |_| rng.next_u32() % 10 ).collect();
    let order_date_array: Vec<i32> = range.clone().map( |_| (rng.next_u32() % 365) as i32 ).collect();
    let amount_array: Vec<f64> = range.clone().map( |_| (rng.next_u32() + 1000) as f64 % 10000f64 / 100.0 ).collect();
    let quantity_array: Vec<u32> = range.clone().map( |_| (rng.next_u32() + 1) % 5 ).collect();

    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(UInt64Array::from(id_array)),
            Arc::new(UInt32Array::from(product_id_array)),
            Arc::new(Date32Array::from(order_date_array)),
            Arc::new(UInt32Array::from(quantity_array)),
            Arc::new(Float64Array::from(amount_array)),
        ]
    )?;

    ctx.register_batch("t1", batch)?;
    Ok(())
}

async fn _test1() -> Result<()> {

    let config = SessionConfig::new().with_target_partitions(1);
    let ctx = SessionContext::new_with_config(config);
    let dir = "/Users/wangzaixiang/workspaces/wangzaixiang/mpp_test/datafusion";
    _ = ctx.register_parquet("sale_orders", format!("{}/sale_orders.parquet", dir), ParquetReadOptions::default()).await;
    _ = ctx.register_parquet("sale_items", format!("{}/sale_items.parquet", dir), ParquetReadOptions::default()).await;
    _ = ctx.register_parquet("purchase_orders", format!("{}/purchase_orders.parquet", dir), ParquetReadOptions::default()).await;
    _ = ctx.register_parquet("purchase_items", format!("{}/purchase_items.parquet", dir), ParquetReadOptions::default()).await;
    _ = ctx.register_parquet("customers", format!("{}/customers.parquet", dir), ParquetReadOptions::default()).await;
    _ =  ctx.register_parquet("customer_tags", format!("{}/customer_tags.parquet", dir), ParquetReadOptions::default()).await;
    _ = ctx.register_parquet("suppliers", format!("{}/suppliers.parquet", dir), ParquetReadOptions::default()).await;
    _ = ctx.register_parquet("tags", format!("{}/tags.parquet", dir), ParquetReadOptions::default()).await;

    // duckdb
    //  select so.order_date, SUM(si.amount) as amount,
    //  	SUM(SUM(si.amount)) over (
    //  		order by so.order_date
    //  		range between to_days( cast( datediff('day',date_trunc('month',so.order_date),  so.order_date)  as integer) ) preceding
    //  			and current row
    //  		)
    //  from sale_items si left join sale_orders so on so.sale_order_id = si.sale_order_id
    //  where so.order_date <= date'2022-01-31'
    //  group by so.order_date;

    let df = ctx.sql(" select so.order_date, SUM(si.amount) as amount,
 	SUM(SUM(si.amount)) over (
 		order by so.order_date
 		-- range between (so.order_date - date_trunc('month',so.order_date) ) preceding
 		rows between 3 preceding
 			and current row
 		),
    SUM(SUM(si.amount)) over (
       order by so.order_date desc
       rows between 3 preceding and current row
    )
 from sale_items si left join sale_orders so on so.sale_order_id = si.sale_order_id
 -- where so.order_date <= date'2022-01-31'
 group by so.order_date
 order by so.order_date;
").await?;

    df.clone().explain(true, false)?.show().await?;
    df.show().await?;

    Ok(())
}