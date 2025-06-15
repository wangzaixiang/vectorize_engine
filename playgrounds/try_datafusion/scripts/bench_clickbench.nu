let ROOT = $"($env.PROCESS_PATH | path dirname)/../../.."
let DATA_DIR = $"($ROOT)/links/datafusion-duckdb-benchmark/clickbench"

let sqls = [
    {
        id: 23,
        "datafusion": "SELECT \"SearchPhrase\", MIN(\"URL\"), MIN(\"Title\"), COUNT(*) AS c, COUNT(DISTINCT \"UserID\")
                        FROM hits
                        WHERE \"Title\" LIKE '%Google%' AND \"URL\" NOT LIKE '%.google.%' AND \"SearchPhrase\" <> ''
                        GROUP BY \"SearchPhrase\" ORDER BY c DESC LIMIT 10",
       "duckdb": "SELECT SearchPhrase, MIN(URL), MIN(Title), COUNT(*) AS c, COUNT(DISTINCT UserID)
                       FROM hits
                       WHERE Title LIKE '%Google%' AND URL NOT LIKE '%.google.%' AND SearchPhrase <> ''
                       GROUP BY SearchPhrase ORDER BY c DESC LIMIT 10"
    },
    {
        id: 24,
        "duckdb": "SELECT * FROM hits WHERE URL LIKE '%google%' ORDER BY EventTime LIMIT 10",
        "datafusion": "SELECT * FROM hits WHERE \"URL\" LIKE '%google%' ORDER BY to_timestamp_seconds\(\"EventTime\"\) LIMIT 10"
    }
]

def main [ --sql: number ] {
    # print "processPath" $processPath

    if ($sqls | where id == $sql | length) == 0 {
        print $sql "not defined"
        exit 1
    }
    let row = $sqls | where id == $sql | first
    print "samply for duckdb" $row.duckdb
    # single file mode, sql 24
    samply record -s -o $"($ROOT)/profiles/clickbench_($sql)_duckdb.prof" duckdb $"($DATA_DIR)/hits.parquet" $"
    pragma threads=1;
    ($row.duckdb);
    "

    print "samply for datafusion" $row.datafusion
    $"
    set datafusion.execution.target_partitions = 1;
    create external table hits stored as parquet location '($DATA_DIR)/hits.parquet';
    ($row.datafusion);
    " | samply record -s -o $"($ROOT)/profiles/clickbench_($sql)_datafusion.prof" datafusion-cli
}

