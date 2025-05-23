use datafusion::prelude::{CsvReadOptions, SessionContext};

// 调试 datafusion 理解 expr eval， 重点是如何通过 template 来特化执行运算，而避免解释开销。
// 总体而言，这个过程相比 duckdb 中的 C++ + template 而言，要更好理解很多。
#[tokio::main]
async fn main() -> datafusion::error::Result<()>{

    let ctx = SessionContext::new();
    _ = ctx.register_csv("access_log", "./playgrounds/try_datafusion/data/access_log.csv", CsvReadOptions::new()).await;

    let q = ctx.sql("select * from access_log where amount > 1000").await?;
    q.clone().explain(false, false)?.show().await?;

    q.show().await?;

    Ok(())
}