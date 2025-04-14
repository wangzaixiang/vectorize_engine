
df-plan.txt 是一个 datafusion 执行 SQL 的查询计划输出文件，该文件中的表格中的 plan 字段是查询计划：
其中每一行是一个 operator，缩进关系表示 operator 的父子关系。

请将改查询计划转换为一个 JSON 的数据结构，格式如下：

```json 
   { "name": "AggregateExec", 
     "mode": "FinalPartitioned", 
   	 "gby": [ "tag_name@0 as tag_name" ], 
   	 "aggr": ["SUM(wt.amount)"],
   	 "metrics": {
   	  	"output_rows": 0,
   	  	"elapsed_compute": "198.667us"
   	 },
   	 "children": [ ]
   }
```

