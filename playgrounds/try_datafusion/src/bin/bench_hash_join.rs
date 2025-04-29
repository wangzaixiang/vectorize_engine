#[tokio::main]
async fn main() {
    test_join().await.unwrap();
}


use datafusion::arrow::array::{Int32Array, Int64Array, RecordBatch};
use datafusion::arrow::datatypes::{DataType, Field, Schema, SchemaRef};
use datafusion::common::JoinType;
use datafusion::execution::TaskContext;
use datafusion::physical_expr::expressions::Column;
use datafusion::physical_plan::ExecutionPlan;
use datafusion::physical_plan::common::collect;
use datafusion::physical_plan::joins::{HashJoinExec, PartitionMode};
use datafusion_datasource::memory::MemorySourceConfig;
use datafusion_datasource::source::DataSourceExec;
/// Hash Join Benchmark
/// design:
/// 1. build side: 20M rows, sale_orders( order_id, customer_id )
/// 2. probe side: 80M rows, sale_items( order_id, amount )
/// on ( order_id )
/// output: ( order_id, customer_id, amount )
use rand::Rng;
use std::sync::Arc;

async fn test_join() -> datafusion::error::Result<()> {
    let task_ctx = prepare_task_ctx();
    let (left_schema, left_batches) = prepare_sale_orders();

    let left = DataSourceExec::from_data_source(MemorySourceConfig::try_new(
        &[left_batches],
        left_schema.clone(),
        None,
    )?);

    let (right_schema, right_batches) = prepare_sale_items();
    let right = DataSourceExec::from_data_source(MemorySourceConfig::try_new(
        &[right_batches],
        right_schema.clone(),
        None,
    )?);

    let on = vec![(
        Arc::new(Column::new_with_schema("order_id", &left_schema)?) as _,
        Arc::new(Column::new_with_schema("order_id", &right_schema)?) as _,
    )];
    let hashjoin = HashJoinExec::try_new(
        left,
        right,
        on,
        None,
        &JoinType::Right,
        None,
        PartitionMode::CollectLeft,
        false,
    )
    .unwrap();
    let stream = hashjoin.execute(0, task_ctx).unwrap();
    let result: Vec<RecordBatch> = collect(stream).await?;
    println!("result batches: {}", result.len());
    println!(
        "total rows = {} ",
        result.iter().map(|t| t.num_rows()).sum::<usize>()
    );
    Ok(())
}

fn prepare_task_ctx() -> Arc<TaskContext> {
    let session_config = datafusion::execution::context::SessionConfig::new();
    Arc::new(TaskContext::default().with_session_config(session_config))
}

fn prepare_sale_orders() -> (SchemaRef, Vec<RecordBatch>) {
    let schema = Arc::new(Schema::new(vec![
        Field::new("order_id", DataType::Int64, false),
        Field::new("customer_id", DataType::Int32, false),
    ]));

    let mut record_batches = Vec::new();
    let total = 20_000_000;
    // let batches = total / 8192 + 1;
    let mut rng = rand::thread_rng();
    for i in (0..total).step_by(8192) {
        let batch_size = 8192.min(total - i);
        let order_id_array: Vec<i64> = (0..batch_size).map(|j| i + j).collect();
        let customer_id_array: Vec<i32> = (0..batch_size)
            .map(|_| rng.gen_range(0..1_000_000))
            .collect();

        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![
                Arc::new(Int64Array::from(order_id_array)),
                Arc::new(Int32Array::from(customer_id_array)),
            ],
        )
        .unwrap();
        record_batches.push(batch);
    }

    (schema, record_batches)
}

fn prepare_sale_items() -> (SchemaRef, Vec<RecordBatch>) {
    let schema = Arc::new(Schema::new(vec![
        Field::new("order_id", DataType::Int64, false),
        Field::new("amount", DataType::Int32, false),
    ]));

    let mut record_batches = Vec::new();
    let total = 80_000_000;
    let mut rng = rand::thread_rng();
    for i in (0..total).step_by(8192) {
        let batch_size = 8192.min(total - i);
        let order_id_array: Vec<i64> = (0..batch_size).map(|j| (i + j) / 4).collect();
        let amount_array: Vec<i32> = (0..batch_size).map(|_| rng.gen_range(0..100)).collect();

        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![
                Arc::new(Int64Array::from(order_id_array)),
                Arc::new(Int32Array::from(amount_array)),
            ],
        )
        .unwrap();
        record_batches.push(batch);
    }

    (schema, record_batches)
}
