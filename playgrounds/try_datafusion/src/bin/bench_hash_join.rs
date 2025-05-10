/// OUT_OF_ORDER: true if the probe side is out of order vs the build side
/// SMALL_BUILD_SET: true if using a small build set
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
use rand::{Rng, RngCore};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

async fn test_join() -> datafusion::error::Result<()> {
    let task_ctx = prepare_task_ctx();
    let out_of_order = if let Ok(str) = std::env::var("OUT_OF_ORDER") {
        str == "true"
    } else {
        false
    };

    let (left_schema, left_batches) =
        if out_of_order { prepare_sale_orders_out_of_order() }
        else { prepare_sale_orders_in_order() };

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

    sleep(Duration::from_secs(1)).await;

    let tm0 = std::time::Instant::now();
    let stream = hashjoin.execute(0, task_ctx).unwrap();
    let result: Vec<RecordBatch> = collect(stream).await?;
    let tm1 = std::time::Instant::now();

    println!(
        "out_of_order = {out_of_order}, time = {:?}, total rows = {} batches = {} ",
        tm1.duration_since(tm0),
        result.iter().map(|t| t.num_rows()).sum::<usize>(),
        result.len(),
    );
    Ok(())
}

fn prepare_task_ctx() -> Arc<TaskContext> {
    let session_config = datafusion::execution::context::SessionConfig::new();
    Arc::new(TaskContext::default().with_session_config(session_config))
}

fn prepare_sale_orders_in_order() -> (SchemaRef, Vec<RecordBatch>) {
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

/// 生成一个 顺序混乱的 sale_orders 数据集
fn prepare_sale_orders_out_of_order() -> (SchemaRef, Vec<RecordBatch>) {
    let schema = Arc::new(Schema::new(vec![
        Field::new("order_id", DataType::Int64, false),
        Field::new("customer_id", DataType::Int32, false),
    ]));

    let mut record_batches = Vec::new();
    let total = 20_000_000usize;

    struct Order {
        order_id: i64,
        customer_id: i32,
        sort: u64
    }

    let mut orders: Vec<Order> = Vec::new();
    let mut rng = rand::thread_rng();
    for i in 0..total {
        let order = Order {
            order_id: i as i64,
            customer_id: (rng.next_u64() % 1_000_000) as i32,
            sort: rng.next_u64()
        };
        orders.push(order);
    }
    orders.sort_by_key( |it| it.sort);

    // let batches = total / 8192 + 1;
    for i in (0..total).step_by(8192) {
        let batch_size = 8192.min(total - i);
        let order_id_array: Vec<i64> = orders[i..i+batch_size]
            .iter()
            .map(|it| it.order_id)
            .collect();
        let customer_id_array: Vec<i32> = orders[i..i+batch_size].iter().map( |it| it.customer_id).collect();

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

    let small_build_set: i64 = if let Ok(str) = std::env::var("SMALL_BUILD_SET") {
        str.parse().unwrap()
    } else {
        20_000_000
    };

    let schema = Arc::new(Schema::new(vec![
        Field::new("order_id", DataType::Int64, false),
        Field::new("amount", DataType::Int32, false),
    ]));

    let mut record_batches = Vec::new();
    let total = 80_000_000;
    let mut rng = rand::thread_rng();
    for i in (0..total).step_by(8192) {
        let batch_size = 8192.min(total - i);
        let order_id_array: Vec<i64> = (0..batch_size).map(|j| (i + j) % small_build_set).collect();
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
