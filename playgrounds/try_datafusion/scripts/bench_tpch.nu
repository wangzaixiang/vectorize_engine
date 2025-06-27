let ROOT = $"($env.PROCESS_PATH | path dirname)/../../.."
let DATA_DIR = $"($ROOT)/links/datafusion-duckdb-benchmark/tpch/data-sf-10"

let sqls = [
    {
        id: 1,
        sql: "select	l_returnflag,	l_linestatus,
             sum(l_quantity) as sum_qty,
             sum(l_extendedprice) as sum_base_price,
             sum(l_extendedprice * (1 - l_discount)) as sum_disc_price,
             sum(l_extendedprice * (1 - l_discount) * (1 + l_tax)) as sum_charge,
             avg(l_quantity) as avg_qty,
             avg(l_extendedprice) as avg_price,
             avg(l_discount) as avg_disc,
             count(*) as count_order
             from	lineitem
             where	l_shipdate <= date '1998-09-02'
             group by	l_returnflag,	l_linestatus
             order by	l_returnflag,	l_linestatus
        "
    },
    {
        id: 7,
        sql: "select	supp_nation,	cust_nation,	l_year,	sum(volume) as revenue
           from	(
              select	n1.n_name as supp_nation,	n2.n_name as cust_nation,	extract(year from l_shipdate) as l_year,	l_extendedprice * (1 - l_discount) as volume
              from	supplier,	lineitem,	orders,	customer,	nation n1,	nation n2
              where	s_suppkey = l_suppkey	and o_orderkey = l_orderkey	and c_custkey = o_custkey	and s_nationkey = n1.n_nationkey
                and c_nationkey = n2.n_nationkey
                and (	(n1.n_name = 'FRANCE' and n2.n_name = 'GERMANY')	or (n1.n_name = 'GERMANY' and n2.n_name = 'FRANCE')	)
                and l_shipdate between date '1995-01-01' and date '1996-12-31'
           ) as shipping
           group by	supp_nation,	cust_nation,	l_year
           order by	supp_nation,	cust_nation,	l_year
        "
    }
]

def main [ --sql: number ] {
    # print "processPath" $processPath

    if ($sqls | where id == $sql | length) == 0 {
        print $sql "not defined"
        exit 1
    }
    let row = $sqls | where id == $sql | first
    print "=== samply for duckdb === "
    # single file mode, sql 24
    $"
    pragma threads=1;
    CREATE VIEW customer AS SELECT * FROM read_parquet\('($DATA_DIR)/customer.parquet'\);
    CREATE VIEW orders AS SELECT * FROM read_parquet\('($DATA_DIR)/orders.parquet'\);
    CREATE VIEW lineitem AS SELECT * FROM read_parquet\('($DATA_DIR)/lineitem.parquet'\);
    CREATE VIEW part AS SELECT * FROM read_parquet\('($DATA_DIR)/part.parquet'\);
    CREATE VIEW partsupp AS SELECT * FROM read_parquet\('($DATA_DIR)/partsupp.parquet'\);
    CREATE VIEW region AS SELECT * FROM read_parquet\('($DATA_DIR)/region.parquet'\);
    CREATE VIEW supplier AS SELECT * FROM read_parquet\('($DATA_DIR)/supplier.parquet'\);
    CREATE VIEW nation AS SELECT * FROM read_parquet\('($DATA_DIR)/nation.parquet'\);

    ($row.sql);
    " | do { print $in; $in} | samply record -s -o $"($ROOT)/profiles/tpch_($sql)_duckdb.prof" duckdb -cmd ".timer on"

    print "=== samply for datafusion ==="
    $"
    set datafusion.execution.target_partitions = 1;

    CREATE EXTERNAL TABLE customer STORED AS PARQUET LOCATION '($DATA_DIR)/customer.parquet';
    CREATE EXTERNAL TABLE orders STORED AS PARQUET LOCATION '($DATA_DIR)/orders.parquet';
    CREATE EXTERNAL TABLE lineitem STORED AS PARQUET LOCATION '($DATA_DIR)/lineitem.parquet';
    CREATE EXTERNAL TABLE part STORED AS PARQUET LOCATION '($DATA_DIR)/part.parquet';
    CREATE EXTERNAL TABLE partsupp STORED AS PARQUET LOCATION '($DATA_DIR)/partsupp.parquet';
    CREATE EXTERNAL TABLE region STORED AS PARQUET LOCATION '($DATA_DIR)/region.parquet';
    CREATE EXTERNAL TABLE supplier STORED AS PARQUET LOCATION '($DATA_DIR)/supplier.parquet';
    CREATE EXTERNAL TABLE nation STORED AS PARQUET LOCATION '($DATA_DIR)/nation.parquet';

    ($row.sql);
    " | do {print $in; $in} | samply record -s -o $"($ROOT)/profiles/tpch_($sql)_datafusion.prof" datafusion-cli
}

