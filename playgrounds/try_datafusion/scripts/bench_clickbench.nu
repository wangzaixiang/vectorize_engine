let ROOT = $"($env.PROCESS_PATH | path dirname)/../../.."
let DATA_DIR = $"($ROOT)/links/datafusion-duckdb-benchmark/clickbench"

let sqls = [
    {
        id: 7,
        datafusion: ("SELECT MIN(`EventDate`::INT::DATE), MAX(`EventDate`::INT::DATE) FROM hits" | str replace -a "`" '"'),
        duckdb: 'SELECT MIN(EventDate), MAX(EventDate) FROM hits'
    },
    {
        id: 22,
        datafusion: ("SELECT `SearchPhrase`, MIN(`URL`), COUNT(*) AS c FROM hits
                      WHERE `URL` LIKE '%google%' AND `SearchPhrase` <> ''
                      GROUP BY `SearchPhrase` ORDER BY c DESC LIMIT 10" | str replace -a '`' '"'),
        duckdb: "SELECT SearchPhrase, MIN(URL), COUNT(*) AS c FROM hits
                 WHERE URL LIKE '%google%' AND SearchPhrase <> ''
                 GROUP BY SearchPhrase ORDER BY c DESC LIMIT 10"
    },
    {
        id: 23,
        datafusion: ("SELECT `SearchPhrase`, MIN(`URL`), MIN(`Title`), COUNT(*) AS c, COUNT(DISTINCT `UserID`)
                        FROM hits
                        WHERE `Title` LIKE '%Google%' AND `URL` NOT LIKE '%.google.%' AND `SearchPhrase` <> ''
                        GROUP BY `SearchPhrase` ORDER BY c DESC LIMIT 10" | str replace -a '`' '"'),
       duckdb: "SELECT SearchPhrase, MIN(URL), MIN(Title), COUNT(*) AS c, COUNT(DISTINCT UserID)
                       FROM hits
                       WHERE Title LIKE '%Google%' AND URL NOT LIKE '%.google.%' AND SearchPhrase <> ''
                       GROUP BY SearchPhrase ORDER BY c DESC LIMIT 10"
    },
    {
        id: 24,
        datafusion: ("SELECT * FROM hits WHERE `URL` LIKE '%google%' ORDER BY to_timestamp_seconds(`EventTime`) LIMIT 10" | str replace -a '`' '"')
        duckdb: "SELECT * FROM hits WHERE URL LIKE '%google%' ORDER BY EventTime LIMIT 10",
    },
    {
        id: 28,
        datafusion: ("SELECT `CounterID`, AVG(length(`URL`::TEXT)) AS l, COUNT(*) AS c FROM hits WHERE `URL` <> '' GROUP BY `CounterID` HAVING COUNT(*) > 100000 ORDER BY l DESC LIMIT 25" | str replace -a '`' '"'),
        duckdb: "SELECT CounterID, AVG(STRLEN(URL)) AS l, COUNT(*) AS c FROM hits WHERE URL <> '' GROUP BY CounterID HAVING COUNT(*) > 100000 ORDER BY l DESC LIMIT 25"
    },
    {
        id: 30,
        datafusion: ("SELECT SUM(`ResolutionWidth`), SUM(`ResolutionWidth` + 1), SUM(`ResolutionWidth` + 2), SUM(`ResolutionWidth` + 3), SUM(`ResolutionWidth` + 4), SUM(`ResolutionWidth` + 5), SUM(`ResolutionWidth` + 6), SUM(`ResolutionWidth` + 7), SUM(`ResolutionWidth` + 8), SUM(`ResolutionWidth` + 9), SUM(`ResolutionWidth` + 10), SUM(`ResolutionWidth` + 11), SUM(`ResolutionWidth` + 12), SUM(`ResolutionWidth` + 13), SUM(`ResolutionWidth` + 14), SUM(`ResolutionWidth` + 15), SUM(`ResolutionWidth` + 16), SUM(`ResolutionWidth` + 17), SUM(`ResolutionWidth` + 18), SUM(`ResolutionWidth` + 19), SUM(`ResolutionWidth` + 20), SUM(`ResolutionWidth` + 21), SUM(`ResolutionWidth` + 22), SUM(`ResolutionWidth` + 23), SUM(`ResolutionWidth` + 24), SUM(`ResolutionWidth` + 25), SUM(`ResolutionWidth` + 26), SUM(`ResolutionWidth` + 27), SUM(`ResolutionWidth` + 28), SUM(`ResolutionWidth` + 29), SUM(`ResolutionWidth` + 30), SUM(`ResolutionWidth` + 31), SUM(`ResolutionWidth` + 32), SUM(`ResolutionWidth` + 33), SUM(`ResolutionWidth` + 34), SUM(`ResolutionWidth` + 35), SUM(`ResolutionWidth` + 36), SUM(`ResolutionWidth` + 37), SUM(`ResolutionWidth` + 38), SUM(`ResolutionWidth` + 39), SUM(`ResolutionWidth` + 40), SUM(`ResolutionWidth` + 41), SUM(`ResolutionWidth` + 42), SUM(`ResolutionWidth` + 43), SUM(`ResolutionWidth` + 44), SUM(`ResolutionWidth` + 45), SUM(`ResolutionWidth` + 46), SUM(`ResolutionWidth` + 47), SUM(`ResolutionWidth` + 48), SUM(`ResolutionWidth` + 49), SUM(`ResolutionWidth` + 50), SUM(`ResolutionWidth` + 51), SUM(`ResolutionWidth` + 52), SUM(`ResolutionWidth` + 53), SUM(`ResolutionWidth` + 54), SUM(`ResolutionWidth` + 55), SUM(`ResolutionWidth` + 56), SUM(`ResolutionWidth` + 57), SUM(`ResolutionWidth` + 58), SUM(`ResolutionWidth` + 59), SUM(`ResolutionWidth` + 60), SUM(`ResolutionWidth` + 61), SUM(`ResolutionWidth` + 62), SUM(`ResolutionWidth` + 63), SUM(`ResolutionWidth` + 64), SUM(`ResolutionWidth` + 65), SUM(`ResolutionWidth` + 66), SUM(`ResolutionWidth` + 67), SUM(`ResolutionWidth` + 68), SUM(`ResolutionWidth` + 69), SUM(`ResolutionWidth` + 70), SUM(`ResolutionWidth` + 71), SUM(`ResolutionWidth` + 72), SUM(`ResolutionWidth` + 73), SUM(`ResolutionWidth` + 74), SUM(`ResolutionWidth` + 75), SUM(`ResolutionWidth` + 76), SUM(`ResolutionWidth` + 77), SUM(`ResolutionWidth` + 78), SUM(`ResolutionWidth` + 79), SUM(`ResolutionWidth` + 80), SUM(`ResolutionWidth` + 81), SUM(`ResolutionWidth` + 82), SUM(`ResolutionWidth` + 83), SUM(`ResolutionWidth` + 84), SUM(`ResolutionWidth` + 85), SUM(`ResolutionWidth` + 86), SUM(`ResolutionWidth` + 87), SUM(`ResolutionWidth` + 88), SUM(`ResolutionWidth` + 89) FROM hits" | str replace -a '`' '"'),
        duckdb: "SELECT SUM(ResolutionWidth), SUM(ResolutionWidth + 1), SUM(ResolutionWidth + 2), SUM(ResolutionWidth + 3), SUM(ResolutionWidth + 4), SUM(ResolutionWidth + 5), SUM(ResolutionWidth + 6), SUM(ResolutionWidth + 7), SUM(ResolutionWidth + 8), SUM(ResolutionWidth + 9), SUM(ResolutionWidth + 10), SUM(ResolutionWidth + 11), SUM(ResolutionWidth + 12), SUM(ResolutionWidth + 13), SUM(ResolutionWidth + 14), SUM(ResolutionWidth + 15), SUM(ResolutionWidth + 16), SUM(ResolutionWidth + 17), SUM(ResolutionWidth + 18), SUM(ResolutionWidth + 19), SUM(ResolutionWidth + 20), SUM(ResolutionWidth + 21), SUM(ResolutionWidth + 22), SUM(ResolutionWidth + 23), SUM(ResolutionWidth + 24), SUM(ResolutionWidth + 25), SUM(ResolutionWidth + 26), SUM(ResolutionWidth + 27), SUM(ResolutionWidth + 28), SUM(ResolutionWidth + 29), SUM(ResolutionWidth + 30), SUM(ResolutionWidth + 31), SUM(ResolutionWidth + 32), SUM(ResolutionWidth + 33), SUM(ResolutionWidth + 34), SUM(ResolutionWidth + 35), SUM(ResolutionWidth + 36), SUM(ResolutionWidth + 37), SUM(ResolutionWidth + 38), SUM(ResolutionWidth + 39), SUM(ResolutionWidth + 40), SUM(ResolutionWidth + 41), SUM(ResolutionWidth + 42), SUM(ResolutionWidth + 43), SUM(ResolutionWidth + 44), SUM(ResolutionWidth + 45), SUM(ResolutionWidth + 46), SUM(ResolutionWidth + 47), SUM(ResolutionWidth + 48), SUM(ResolutionWidth + 49), SUM(ResolutionWidth + 50), SUM(ResolutionWidth + 51), SUM(ResolutionWidth + 52), SUM(ResolutionWidth + 53), SUM(ResolutionWidth + 54), SUM(ResolutionWidth + 55), SUM(ResolutionWidth + 56), SUM(ResolutionWidth + 57), SUM(ResolutionWidth + 58), SUM(ResolutionWidth + 59), SUM(ResolutionWidth + 60), SUM(ResolutionWidth + 61), SUM(ResolutionWidth + 62), SUM(ResolutionWidth + 63), SUM(ResolutionWidth + 64), SUM(ResolutionWidth + 65), SUM(ResolutionWidth + 66), SUM(ResolutionWidth + 67), SUM(ResolutionWidth + 68), SUM(ResolutionWidth + 69), SUM(ResolutionWidth + 70), SUM(ResolutionWidth + 71), SUM(ResolutionWidth + 72), SUM(ResolutionWidth + 73), SUM(ResolutionWidth + 74), SUM(ResolutionWidth + 75), SUM(ResolutionWidth + 76), SUM(ResolutionWidth + 77), SUM(ResolutionWidth + 78), SUM(ResolutionWidth + 79), SUM(ResolutionWidth + 80), SUM(ResolutionWidth + 81), SUM(ResolutionWidth + 82), SUM(ResolutionWidth + 83), SUM(ResolutionWidth + 84), SUM(ResolutionWidth + 85), SUM(ResolutionWidth + 86), SUM(ResolutionWidth + 87), SUM(ResolutionWidth + 88), SUM(ResolutionWidth + 89) FROM hits"
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
    $"
    pragma threads=1;
    create view hits as select * replace \( epoch_ms\( EventTime * 1000 \) as EventTime, Date'1970-01-01' + INTERVAL \(EventDate\) DAYS as EventDate \)
        from read_parquet\( '($DATA_DIR)/hits.parquet', binary_as_string=true \);

    ($row.duckdb);
    " | samply record -s -o $"($ROOT)/profiles/clickbench_($sql)_duckdb.prof" duckdb -echo -cmd ".timer on"

    print "samply for datafusion" $row.datafusion
    $"
    set datafusion.execution.target_partitions = 1;
    create external table hits stored as parquet location '($DATA_DIR)/hits.parquet';
    ($row.datafusion);
    " | samply record -s -o $"($ROOT)/profiles/clickbench_($sql)_datafusion.prof" datafusion-cli
}

