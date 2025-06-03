use std::sync::Arc;
use chrono::NaiveDate;
use datafusion::arrow::array::{Date32Array, Float32Array, Float64Array, Int32Array, Int64Array, RecordBatch, UInt32Array, UInt64Array};
use datafusion::arrow::datatypes::{DataType, Field, Schema};
use datafusion::error::{DataFusionError, Result};
use datafusion::prelude::{ParquetReadOptions, SessionConfig, SessionContext};
use datafusion::sql::sqlparser::ast::Ident;
use datafusion_datasource::memory::MemorySourceConfig;
use datafusion_datasource::source::DataSourceExec;
use rand::RngCore;

// fn main(){
//
// }

/// WindowAggExec + PlainAggregateWindowExpr + Accumulator
#[tokio::test]
async fn test_sum_1() -> Result<()> {
    let mut ctx = SessionContext::new();
    _ = prepare(&mut ctx).await?;

    let df2 = ctx.sql("select *, sum(amount) over (partition by product_id) as product_amounts from t1").await?;
    df2.show().await?;
    Ok(())
}

/// WindowAggExec + SlidingAggregateWindowExpr + Accumulator
#[tokio::test]
async fn test_sum_2() -> Result<()> {
    let mut ctx = SessionContext::new();
    _ = prepare(&mut ctx).await?;

    let df2 = ctx.sql("select *, sum(amount) over (partition by product_id rows between current row and unbounded following) as amounts1 from t1").await?;

    df2.show().await?;
    Ok(())
}



/// BoundedWindowAggExec + SlidingAggregateWindowExpr + Accumulator(sliding)
#[tokio::main]
// async fn test_sum_3() -> Result<()> {
async fn main() -> Result<()> {
    let config = SessionConfig::new().with_batch_size(8);
    let mut ctx = SessionContext::new_with_config(config);
    // let mut ctx = SessionContext::new();
    _ = prepare(&mut ctx).await?;

    let sql = "select *, sum(amount) over (order by order_date range between interval '1 days' preceding and interval '1 days' following) as slide_amounts from t1";
    // let sql = "select *,sum(amount) over (partition by product_id order by order_date rows 1 preceding) as amounts1 from t1";
    let df2 = ctx.sql(sql).await?;
    // df2.clone().explain(false, false)?.show().await?;

    println!("sql:{}", sql);
    _ = df2.show().await;
    Ok(())
}

/// BoundedWindowAggExec + PlainAggregateWindowExpr + Accumulator
#[tokio::test]
async fn test_sum_4() -> Result<()> {
    let mut ctx = SessionContext::new();
    _ = prepare(&mut ctx).await?;

    let df2 = ctx.sql("select *, \
    sum(amount) over (partition by product_id order by order_date rows between unbounded preceding and 1 following) as amounts1 from t1").await?;

    _ = df2.show().await;
    Ok(())
}


/// case 3: BoundedWindowAggExec + StandardWindowExpr + PartitionEvaluator::evaluate
#[tokio::test]
async fn test_rank_1() -> Result<()> {
    let mut ctx = SessionContext::new();
    _ = prepare(&mut ctx).await?;

    let df2 = ctx.sql("select *, rank() over (partition by product_id order by order_date) as rank1 from t1").await?;
    _ = df2.show().await;
    Ok(())
}

/// ! frame is defined but not used
#[tokio::test]
async fn test_rank_2() -> Result<()> {
    let mut ctx = SessionContext::new();
    _ = prepare(&mut ctx).await?;

    let df2 = ctx.sql("select *, rank() over (partition by product_id order by order_date rows 1 preceding) as rank1 from t1").await?;
    _ = df2.show().await;
    Ok(())
}

/// WindowAggExec + StandardWindowExpr + PartitionEvaluator::evaluate_all
#[tokio::test]
async fn test_ntile_1() -> Result<()> {
    let mut ctx = SessionContext::new();
    _ = prepare(&mut ctx).await?;

    let df2 = ctx.sql("select *, ntile(4) over () as ntile from t1").await?;
    _ = df2.show().await?;
    Ok(())
}

fn parse_date(str: &str) -> Result<i32> {
    let date = NaiveDate::parse_from_str(str, "%Y-%m-%d")
        .map_err(|e| DataFusionError::Execution(e.to_string()))?;
    let days = date.signed_duration_since(NaiveDate::from_ymd(1970,1,1)).num_days();
    Ok(days as i32)
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

    struct Record {
        id: u64,
        product_id: u32,
        order_date: i32,
        quantity: u32,
        amount: f64,
    }
    impl Record {
        fn new(id: u64, product_id: u32, order_date: i32, quantity: u32, amount: f64) -> Self {
            Record {
                id, product_id, order_date, quantity, amount
            }
        }
    }

    let records: Vec<Record> = vec![
        Record::new(0, 1, parse_date("2024-01-01")?, 1, 10.0 ),
        Record::new(1, 2, parse_date("2024-01-02")?, 2, 20.0 ),
        Record::new(2, 3, parse_date("2024-01-02")?, 3, 30.0 ),
        Record::new(3, 4, parse_date("2024-01-04")?, 4, 40.0 ),
        Record::new(4, 2, parse_date("2024-01-06")?, 5, 50.0 ),
        Record::new(5, 3, parse_date("2024-01-07")?, 6, 60.0 ),
        Record::new(6, 4, parse_date("2024-01-08")?, 7, 70.0 ),
        Record::new(7, 3, parse_date("2024-01-08")?, 8, 80.0 ),
        Record::new(8, 4, parse_date("2024-01-09")?, 9, 90.0 ),
        Record::new(9, 4, parse_date("2024-01-12")?, 10, 100.0 ),
    ];

    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(UInt64Array::from( records.iter().map(|r|r.id).collect::<Vec<u64>>() )),
            Arc::new(UInt32Array::from( records.iter().map(|r|r.product_id).collect::<Vec<u32>>() )),
            Arc::new(Date32Array::from( records.iter().map(|r|r.order_date).collect::<Vec<i32>>() )),
            Arc::new(UInt32Array::from( records.iter().map(|r|r.quantity).collect::<Vec<u32>>() )),
            Arc::new(Float64Array::from( records.iter().map(|r|r.amount).collect::<Vec<f64>>() )),
        ]
    )?;

    let batches: Vec<RecordBatch> = (0..1).map( |_| batch.clone() ).collect();

    let df = ctx.read_batches( batches )? ;
    ctx.register_table("t1", df.into_view())?;

    // ctx.register_batch("t1", batch)?;
    Ok(())
}