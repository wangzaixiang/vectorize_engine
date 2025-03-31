# QIR Engine prototype Dev Plan

## QIR parser
1. 使用 rust 源代码来构建 QIR
2. 算子定义使用 Macro + JSON 来定义，参考：

   ```rust

   struct Table {
     name: String,
     colummns: Vec<Column>
   };

   struct Column {
     name: String,
     `type`: DataType
   }

   // table! macro generate a Table struct
   let order_item = table! {
    name = "order_item",
    columns = [
        column! {name = "order_item_id", `type` = u64 },
        column! {name = "order_date", type = date },
        column! {name = "user_id", type = u32 },
        column! {name = "product_id", type = u32 },
        column! {name = "quantity", type = u32 },
        column! {name = "amount", type = decimal }
    ]
   }
   ```   
   使用该方式，我们可以快速的实现一个原形的 QIR builder，而避免需要开发一个 QIR parser的工作量。

## demo tables
    使用 Arrow API，创建 arrow 文件，存储 demo tables:
    - orders
    - order_items
    - products
    - customers
    - catagories
    - tags
    - customer_tags

    后续开发过程中，可以使用这些文件作为数据源。
    避免使用 csv 文件是便于忽略读写源表的执行开销，避免对算子的性能测试进行影响。

## pipeline 执行引擎
    pipeline 是 QIR 的核心，其调度涉及到 分区、依赖关系等。这个开发过程是逐步迭代的。

## 算子
    prototype 阶段先以简单的方式实现算子的原型，在单元测试能通过后，再进行性能改造，选择向量算法进行优化。


# TODO LIST
- [ ] TDD test case: build QIR for example-1 using rust macro
  - 自引用的复杂对象树构建，加深对 lifetime 和 self-referential struct 的理解。
  - 使用 Rc<T> 还是 &T?
  - 学习 rust macro
- [ ] 熟悉 arrow api，尝试创建 `users(id, name, birthday, email) ` dataset
- [ ] pass test: run a simple pipeline for example-1 (no partition)