use datafusion::error::Result;
use datafusion::prelude::{CsvReadOptions, SessionContext};

#[tokio::main]
async fn main() -> Result<()> {

    let ctx = SessionContext::new();
    let root = "playgrounds/try_datafusion";
    ctx.register_csv("orders", format!("{root}/{}","data/orders.csv"), CsvReadOptions::new()).await?;
    ctx.register_csv("products", format!("{root}/{}","data/products.csv"), CsvReadOptions::new()).await?;

    let df = ctx.sql("select p.product_name, count(1) as cnt, sum(o.quantity) as quantity, sum(o.amount) as amount from orders o left join products p
        on o.product_id = p.product_id
        group by p.product_name
        -- order by 1
    ").await?;

    df.clone().explain(false, true)?.show().await?;

    df.show().await?;

    Ok(())
}

//调试 datafusion 源代码，理解执行计划，及下列算子的执行原理。
//  ProjectionExec
//      AggregateExec mode=FinalPartitioned
//          CoalesceBatchesExec
//              RepartitionExec(Hash(product_name, 10))
//                  AggregateExec model=Partial
//                      CoalesceBatchesExec
//                          HashJoinExec mode=Partitioned
//                              CoalesceBatchesExec  <- RepartitionExec  <- RepartitionExec <- DataSourceExec(orders.csv)
//                              CoalesceBatchesExec  <- RepartitionExec(Hash(product_id, 10))  <- RepartitionExec(RoundRobinBatch(10)) <- DataSourceExec(products.csv)
//