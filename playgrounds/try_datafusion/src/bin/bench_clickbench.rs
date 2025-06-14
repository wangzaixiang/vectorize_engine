use std::fmt::format;
use std::sync::Arc;
use datafusion::common::DataFusionError;
use datafusion::datasource::file_format::parquet::ParquetFormat;
use datafusion::datasource::listing::ListingOptions;
use datafusion::prelude::{ParquetReadOptions, SessionConfig, SessionContext};
use datafusion::error::Result;

// Prepare dataset
//
// this program requires files under datafusion-duckdb-benchmark/clickbench/hits.parquet
//  original project: https://github.com/wangzaixiang/datafusion-duckdb-benchmark.git
// and runs clickbench/setup.sh to download the hits dataset from https://datasets.clickhouse.com/hits_compatible
// make sure checkout the project, runs clickbench/setup.sh, and create a symbolic link under links/datafusion-duckdb-benchmark
const DIR_DATAFUSION_DUCKDB_BENCHMARK: &str = "links/datafusion-duckdb-benchmark";

#[tokio::main]
async fn main() -> Result<()> {

    let ctx = prepare_context().await?;

    let _sql23 = r###"SELECT "SearchPhrase", MIN("URL"), MIN("Title"), COUNT(*) AS c, COUNT(DISTINCT "UserID")
    FROM hits
    WHERE "Title" LIKE '%Google%' AND "URL" NOT LIKE '%.google.%' AND "SearchPhrase" <> ''
    GROUP BY "SearchPhrase" ORDER BY c DESC LIMIT 10"###;

    let sql24 = r###"SELECT * FROM hits WHERE "URL" LIKE '%google%' ORDER BY to_timestamp_seconds("EventTime") LIMIT 10"###;

    let df = ctx.sql(sql24).await?;
    // df.clone().explain(true, true)?.show().await?;

    let tm0 = std::time::Instant::now();
    df.show().await?;

    let tm1 = std::time::Instant::now();
    println!("Total execution time: {:?}", tm1.duration_since(tm0));

    Ok(())
}

// sql23 in duckdb
// single parquet mode
// duckdb cost: 12.265s: scan_filter(12s)
// df cost:     14.649s: scan(12.998) + filter(1.448)
// in single mode: Title@0 LIKE %Google% AND URL@2 NOT LIKE %.google.%

// multi parquet mode
// duckdb multi cost: 11.55s
// df multi cost: 23.47s
// 主要开销：12s, most time:  CAST(Title@0 AS Utf8View) LIKE %Google% AND CAST(URL@2 AS Utf8View) NOT LIKE %.google.%
// multi parquet 的列上为 BinaryView，引入了一次 CAST Utf8View 的操作。
// 主要开销在 cast 操作上，引入了大量的 from_utf8 的计算。

// SQL24
// df: 40s, scan without filter
// duckdb: 8.89s

async fn prepare_context() -> Result<SessionContext> {
    let config = SessionConfig::new().with_target_partitions(1);
    let context = SessionContext::new_with_config(config);

    let multi: bool = std::env::var("MULTI").or::<DataFusionError>(Ok("false".to_string()))?.parse().or::<DataFusionError>(Ok(false))?;

    if multi == false {
        // single file
        println!("single parquet mode");
        context.register_parquet("hits",
                                 &format!("{}/clickbench/hits.parquet", DIR_DATAFUSION_DUCKDB_BENCHMARK),
                                 ParquetReadOptions::default()).await?;
    }
    else {
        // multi file
        println!("multiple parquet mode");
        context.register_listing_table("hits",
                                       &format!("{}/clickbench/hits_multi", DIR_DATAFUSION_DUCKDB_BENCHMARK),
                                       ListingOptions::new(Arc::new(
                                           ParquetFormat::default()
                                       )).with_file_extension(".parquet"),
                                       None, None
        ).await?;
    }
    Ok(context)
}
