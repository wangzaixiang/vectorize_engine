# 关系向量化实验引擎

## 1. 项目简介
在学习 duckdb 的过程中，对 Physical Execution 我萌生了一个想法：构建一个独立的向量化引擎，支持各种关系代数操作，满足如下的特性：
- 作为一个独立的 execution engine, 自身不处理 SQL 的解析、逻辑计划、优化等工作，只负责执行物理计划。
- 理论上具备与各种关系数据库前端对接的能力，可作为 duckdb 等数据库的 physical execution engine。
- 自底向上的方式设计每一个算子，性能优化。可以探索各种优化技术的实现。
- 构建一个 Typed IR, 作为执行引擎的输入，该 IR 满足：
  - 可读性。面向开发人员，利于理解。
  - 强类型。具有静态类型检查的能力，减少运行时的错误和开销。
  - 便于高效的解释执行
  - 未来有可能 JIT 执行。
  - 基于向量、pipeline 的执行模型。

## 2. 设计思路

以一个简单的 SQL 为例：
```sql 
select name, count(freight), sum(freight) 
from sale_orders so 
left join customers c on c.customer_id = so.customer_id 
where gender = 'M' and name like 'abc%'  and freight > 10 and freight < 50 
group by name
```

该 SQL 的执行计划表示如下：(伪代码，将进一步抽象为 Typed IR)
```
// scan( customers ) |> filter |> build_hash
pipeline1: Pipeline =
    // Vector<(customer_id:i32, name:string, gender:string)>
    v1 = table_scan :table = "customers"
            :columns = 
                "customer_id": { data_type: "i32", nullable: false, ordered: 0, unique: true },
                "name": { data_type: "string", nullable: false, ordered: 0, unique: false },
                "gender": { data_type: "string", nullable: false, ordered: 0, unique: false }
            :minmaxes = 
                "gender": ["M", "M"],
                "name": ["abc", "abd"]
    // Vector<(customer_id:i32, name:string)>            
    v2 = filter :input = v1
            :expr = $in.gender == "M" && ($in.name >= "abc" && $in.name < "abd")
            :projection = [ "customer_id", "name" ]
   
    // Tuple
    minmax = aggregate :input = v2.customer_id
            :aggregates = 
                "min": min
                "max": max
            
    ht1 = build_hash :input = v2
            :key = "customer_id"
            :value = "name"        


// scan( sale_orders ) |> filter | hash_join |> project |> hash_group_by
pipelin2: Pipeline =
    // Vector<(customer_id:i32, freight:f64)>
    v1 = table_scan :table = "sale_orders"
            :columns = 
                "customer_id": { data_type: "i32", nullable: false, ordered: 0, unique: true },
                "freight": { data_type: "f64", nullable: false, ordered: 0, unique: false }
            :minmaxes = 
                "freight": [10, 50]
                "customer_id": [pipeline1.minmax.min, pipeline1.minmax.max]
                
    // Vector<(customer_id:i32, freight:f64)>            
    v2 = filter :input = v1
            :expr = $in.freight > 10 && $in.freight < 50
            :projection = [ "customer_id", "freight" ]
            
    // Vector<(customer_id:i32, name:string, freight:f64)>
    v3 = hash_left_join 
            :ht = pipeline1.ht1 
            :key = v2.customer_id
            :projection = [ v2.customer_id, $ht.name, v2.freight ]
    
    // Vector<(name:string, count:i64, sum:i64)>        
    v4 = hash_group_by :input = v3
            :group_by = [ "name" ]
            :aggregates = 
                "count": count($in.freight)
                "sum": sum($in.freight)

```

目标：
1. 向量类型
2. pipeline
3. 非向量类型： HashTable, Tuple 等

优化:
1. scan
   - minmax filter
   - bloom filter
2. join
   - hash join (hashtable in memory)
   - hash join (hashtable on disk for large right table)
   - right table is sorted.
   - filter push down from right to left
3. filter push down
4. window function
5. pre-aggregate
6. deleted bitmap

## Typed AST
首先，我们需要定义一个 Typed AST， 用于描述 执行计划：
- 向量
  - 元素类型
  - 是否可空
  - 是否有序
  - 是否唯一
- 关系
  - 列定义
- Pipeline
  - 变量
- 变量: Term
- 类型：Type
- 表达式：
  - Select
  - Apply


