
qp 是新设计的一种用于描述关系代数查询的中间语言，SQL 查询可以翻译称为 qp 代码，然后再解释或者编译执行。

# 示例
## 示例一
```sql
-- table: users(id, name, birthday, email)
select name, email from users where birthday > '1990-01-01';
```

其对应的 qp 代码如下：
```qp 

// type users 是一个表类型信息，用于为后面的代码提供类型检查，例如字段名是否正确，数据类型是否正确等。
type users = table {
    name = "users",
    columns = [
        {name = "id", type = "int"},
        {name = "name", type = "string"},
        {name = "birthday", type = "date"},
        {name = "email", type = "string"}
    ]    
}; 

// source1 是一个 Source operator, 其输出类型为： DataFrame {
//     columns = [
//         {name = "name", type = "string"},
//         {name = "birthday", type = "date"}
//         {name = "email", type = "string"}
//     ]
// }
let source1 = scan(
    name = "users", // table name
    table = users, 
    output = ["name", "birthday", "email"] ); // 从 users 表中读取数据

/// filter1 是一个 ordinary operator, 其输出类型为： DataFrame {
///     columns = [
///         {name = "name", type = "string"},
///         {name = "email", type = "string"}
///     ]
/// }
let filter1 = filter(input = source1, 
    predicator = |row| { row.birthday > date'1990-01-01' },     // 过滤出 birthday > '1990-01-01' 的数据 
    output = ["name", "email"] );   // 输出 name 和 email 字段

/// sink1 是一个 sink operator, 其输出类型与输入类型相同，起到一个收集的作用
let sink1 = identity_sink(input = filter1); 

let pipeline1 = pipeline {
    source = source1,
    operators = [filter1],
    sink = sink1
};

// graph 是一个 topology 类型，用于描述整个查询的拓扑结构，也是单个 qp 文件的最终输出结果。
let graph = topology( main = pipeline1 );
```

## 示例二
```sql
--- table: order_item(order_item_id: u64, order_date: date, product_id: u32, quantity: u32, amount: decimal)
--- table: product(product_id: u32, product_name: string, category_id: u32)

select product_name, sum(quantity), sum(amount)
from order_item join product on order_item.product_id = product.product_id
group by product_name
```
对应的 qp 代码如下：
```qp
// 定义表结构类型
type order_item = table {
    name = "order_item",
    columns = [
        {name = "order_item_id", type = "u64"},
        {name = "order_date", type = "date"},
        {name = "product_id", type = "u32"},
        {name = "quantity", type = "u32"},
        {name = "amount", type = "decimal"}
    ]
};

type product = table {
    name = "product",
    columns = [
        {name = "product_id", type = "u32"},
        {name = "product_name", type = "string"},
        {name = "category_id", type = "u32"}
    ]
};

// 源操作符
let order_item_source = scan(
    name = "order_item",
    table = order_item,
    output = ["product_id", "quantity", "amount"]  // 只需要参与计算的字段
);

let product_source = scan(
    name = "product_scan",
    table = product,
    output = ["product_id", "product_name"]       // 只需要关联字段和分组字段
);

let ht_build1 = build_hash_table(
	input = product_source,
	key = [ "product_id" ] );

let ht_lookup1 = lookup_hash_table( ht = ht_build1, input = order_item_source, key = ["product_id"],
	output = [ ["quantity", "amount"], // field from lookup side
			   ["product_name"] ]	// field from build side
 );

let hash_aggr1 = hash_aggregator( input = h1_lookup1, 
	group_by = ["product_name"],
	aggregators = [
		sum_aggregator( name="total_quantity", field = "quantity" ),
		sum_aggregator( name="total_amount", field = "amount" )
	],
	output = [ "product_name", "total_quantity", "total_amount" ]
);


let pipeline1 = pipeline( source = product_source, sink = ht_build1 );
let pipeline2 = pipeline( source = order_item_source, operators = [ht_lookup1], sink = hash_aggr1, 
	parent = [pipeline1] );	// pipeline2 依赖 pipeline1, 仅当 pipeline1 执行完成后，才能开始执行


let graph = topology(main = pipeline2);
```

## 示例3
```sql
-- table: order_item(order_item_id, order_date, user_id, product_id, quantity, amount)
-- table: users(user_id, name, sex, province, city)
select province, city, count(order_item_id), sum(quantity), sum(amount)
from order_item left join users on order_item.user_id = users.user_id
where order_date >= '2025-01-1' and sex = 'F'
group by province, city
```

```qp
// 定义表结构类型
type order_item = table {
    name = "order_item",
    columns = [
        {name = "order_item_id", type = "u64"},
        {name = "order_date", type = "date"},
        {name = "user_id", type = "u32"},
        {name = "product_id", type = "u32"},
        {name = "quantity", type = "u32"},
        {name = "amount", type = "decimal"}
    ]
};

type users = table {
    name = "users",
    columns = [
        {name = "user_id", type = "u32"},
        {name = "name", type = "string"},
        {name = "sex", type = "string"},
        {name = "province", type = "string"},
        {name = "city", type = "string"}
    ]
};

// 源操作符和过滤（优化版）
let users_scan = scan(
    name = "users",
    table = users,
    output = ["user_id", "sex", "province", "city"]
);

let users_filter = filter(
    input = users_scan,
    predicator = |row| { row.sex == "F" },  // 提前过滤女性用户
    output = ["user_id", "province", "city"]
);

let ht_build = build_hash_table(
    input = users_filter,
    key = ["user_id"]
);

let order_item_scan = scan(
    name = "order_item",
    table = order_item,
    output = ["user_id", "order_item_id", "quantity", "amount", "order_date"]
);

let order_filter = filter(
    input = order_item_scan,
    predicator = |row| { row.order_date >= date'2025-01-01' },
    output = ["user_id", "order_item_id", "quantity", "amount"]
);

let ht_lookup = lookup_hash_table(
    ht = ht_build,
    input = order_filter,
    key = ["user_id"],
    output = [
        ["order_item_id", "quantity", "amount"],  // 订单表字段
        ["province", "city"]                     // 用户表字段（已过滤）
    ],
    join_type = "inner"  // 改为内连接（因为用户表已过滤）
);

let hash_aggr = hash_aggregator(
    input = ht_lookup,
    group_by = ["province", "city"],
    aggregators = [
        count_aggregator(name = "total_orders", field = "order_item_id"),
        sum_aggregator(name = "total_quantity", field = "quantity"),
        sum_aggregator(name = "total_amount", field = "amount")
    ],
    output = ["province", "city", "total_orders", "total_quantity", "total_amount"]
);

// 流水线定义（优化版）
let pipeline1 = pipeline(
    source = users_scan,
    operators = [users_filter],
    sink = ht_build
);

let pipeline2 = pipeline(
    source = order_item_scan,
    operators = [order_filter, ht_lookup, hash_aggr],
    sink = hash_aggr,
    parent = [pipeline1]  // 强依赖
);

let graph = topology(main = pipeline2);
```

# 示例4 
```sql
-- table: order_item(order_item_id, order_date, user_id, product_id, quantity, amount)
-- table: users(user_id, name, sex, province, city)
select province, city, count(order_item_id), sum(quantity), sum(amount)
from order_item left join users on order_item.user_id = users.user_id
where order_date >= '2025-01-1' and sex = 'F'
group by province, city
having count(order_item_id) > 10
```

等价的 qp 代码：
```qp
// 定义表结构类型（与示例3相同）
type order_item = table {
    name = "order_item",
    columns = [
        {name = "order_item_id", type = "u64"},
        {name = "order_date", type = "date"},
        {name = "user_id", type = "u32"},
        {name = "product_id", type = "u32"},
        {name = "quantity", type = "u32"},
        {name = "amount", type = "decimal"}
    ]
};

type users = table {
    name = "users",
    columns = [
        {name = "user_id", type = "u32"},
        {name = "name", type = "string"},
        {name = "sex", type = "string"},
        {name = "province", type = "string"},
        {name = "city", type = "string"}
    ]
};

// 源操作符和过滤（继承优化逻辑）
let users_scan = scan(
    name = "users",
    table = users,
    output = ["user_id", "sex", "province", "city"]
);

let users_filter = filter(
    input = users_scan,
    predicator = |row| { row.sex == "F" },
    output = ["user_id", "province", "city"]
);

let ht_build = build_hash_table(
    input = users_filter,
    key = ["user_id"]
);

let order_item_scan = scan(
    name = "order_item",
    table = order_item,
    output = ["user_id", "order_item_id", "quantity", "amount", "order_date"]
);

let order_filter = filter(
    input = order_item_scan,
    predicator = |row| { row.order_date >= date'2025-01-01' },
    output = ["user_id", "order_item_id", "quantity", "amount"]
);

let ht_lookup = lookup_hash_table(
    ht = ht_build,
    input = order_filter,
    key = ["user_id"],
    output = [
        ["order_item_id", "quantity", "amount"],
        ["province", "city"]
    ],
    join_type = "inner"
);

// 新增 HAVING 过滤阶段
let hash_aggr = hash_aggregator(
    input = ht_lookup,
    group_by = ["province", "city"],
    aggregators = [
        count_aggregator(name = "total_orders", field = "order_item_id"),
        sum_aggregator(name = "total_quantity", field = "quantity"),
        sum_aggregator(name = "total_amount", field = "amount")
    ],
    output = ["province", "city", "total_orders", "total_quantity", "total_amount"]
);

let having_filter = filter(
    input = hash_aggr,
    predicator = |row| { row.total_orders > 10 },  // HAVING 条件
    output = ["province", "city", "total_orders", "total_quantity", "total_amount"]
);

// 流水线定义
let pipeline1 = pipeline(
    source = users_scan,
    operators = [users_filter],
    sink = ht_build
);

let pipeline2 = pipeline(
    source = order_item_scan,
    operators = [order_filter, ht_lookup, hash_aggr, having_filter],
    sink = having_filter,
    parent = [pipeline1]
);

let graph = topology(main = pipeline2);
```

# 算子说明

```markdown
1. filter 对输入 DataFrame 进行过滤操作
   - input: 输入 DataFrame
   - predicator: 过滤函数，返回 true 表示保留，返回 false 表示丢弃
   - output: 输出字段列表

2. build_hash_table 构建哈希表，用于关联操作，与 lookup_hash_table 配合使用
   - input: 输入 DataFrame
   - key: 哈希表的 key 列
   - output: 输出字段列表

3. lookup_hash_table 从哈希表中查找数据，用于关联操作
   - ht: 哈希表，由 build_hash_table 构建
   - input: 输入 DataFrame
   - key: 关联字段
   - output: 输出字段列表
   - join_type: 连接类型，inner/left/right/semi/anti-semi/crossjoin 等
```

请参考上述的算子说明，补充示例中用到的算子的文档，输出为 markdown 格式。