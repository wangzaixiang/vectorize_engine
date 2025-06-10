let DATA_DIR = $"/Volumes/wangzx-sandisk/workspaces/github.com/datafusion-duckdb-benchmark/clickbench"
let SQL_24_DUCKDB = "SELECT * FROM hits WHERE URL LIKE '%google%' ORDER BY EventTime LIMIT 10"
let SQL_24_DF = "SELECT * FROM hits WHERE \"URL\" LIKE '%google%' ORDER BY to_timestamp_seconds\(\"EventTime\"\) LIMIT 10"

let SQL_24 = {
    "duckdb": "SELECT * FROM hits WHERE URL LIKE '%google%' ORDER BY EventTime LIMIT 10",
    "datafusion": "SELECT * FROM hits WHERE \"URL\" LIKE '%google%' ORDER BY to_timestamp_seconds\(\"EventTime\"\) LIMIT 10"
}
let SQL_23 = {
    "duckdb": "",
    "datafusion": "",
}

# single file mode, sql 24
samply record -s -o clickbench_duckdb.prof duckdb $"($DATA_DIR)/hits.parquet" $"
pragma threads=1;
($SQL_24_DUCKDB);
"

$"
set datafusion.execution.target_partitions = 1;
create external table hits stored as parquet location '($DATA_DIR)/hits.parquet';
($SQL_24_DF);
" | samply record -s -o clickbench_datafusion.prof datafusion-cli
