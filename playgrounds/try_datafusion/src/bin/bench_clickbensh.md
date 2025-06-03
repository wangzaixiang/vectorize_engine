
```bash, ingore

let DATA_DIR = $"($env.HOME)/workspaces/github.com/datafusion-duckdb-benchmark/clickbench"

# single file mode
duckdb $"($DATA_DIR)/hits.parquet" "
pragma threads=1; 
SELECT SearchPhrase, MIN(URL), MIN(Title), COUNT(*) AS c, COUNT(DISTINCT UserID) 
FROM hits 
WHERE Title LIKE '%Google%' AND URL NOT LIKE '%.google.%' AND SearchPhrase <> '' 
GROUP BY SearchPhrase 
ORDER BY c DESC 
LIMIT 10;"


# multi file mode
duckdb my.duckdb ("
 pragma threads=1;
 CREATE VIEW if not exists hits AS SELECT *
 	REPLACE
 	(epoch_ms(EventTime * 1000) AS EventTime,
 	 DATE '1970-01-01' + INTERVAL (EventDate) DAYS AS EventDate)
 FROM read_parquet('" + $DATA_DIR + "/hits_multi/hits_*.parquet', binary_as_string=True);
 
 SELECT SearchPhrase, MIN(URL), MIN(Title), COUNT(*) AS c, COUNT(DISTINCT UserID)
 FROM hits
 WHERE Title LIKE '%Google%' AND URL NOT LIKE '%.google.%' AND SearchPhrase <> ''
 GROUP BY SearchPhrase 
 ORDER BY c DESC 
 LIMIT 10;
 ")

```