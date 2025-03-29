use crate::{table, column, scan, filter, identity, pipeline};
use crate::qir::*;

#[test]
fn test_example1() {
    // 1. 定义表结构
    let users = table! {
        name: "users",
        columns: [
            column! { name = "id", data_type = I64 },
            column! { name = "name", data_type = String },
            column! { name = "birthday", data_type = Date },
            column! { name = "email", data_type = String },
        ],
    };

    // 2. 创建 scan 算子
    let users_scan = scan! {
        name: "users_scan",
        table: users,
        output: ["name", "birthday", "email"]
    };

    // 3. 创建 filter 算子
    let users_filter = filter! {
        input: users_scan,
        predicate: "row.birthday > '1990-01-01'",
        output: ["name", "email"]
    };

    // 4. 创建 identity sink
    let sink = identity! {
        input: users_filter
    };

    // 5. 创建 pipeline
    let pipeline = pipeline! {
        source: users_scan,
        operators: [users_filter],
        sink: sink
    };

    // 6. 创建 topology
    let topology = Topology {
        main: std::rc::Rc::new(pipeline)
    };

    // 7. 验证结果
    assert_eq!(users.name, "users");
    assert_eq!(users.columns.len(), 4);
    
    // 验证 scan 算子
    assert_eq!(users_scan.name, "users_scan");
    assert_eq!(users_scan.output, vec!["name", "birthday", "email"]);
    
    // 验证 filter 算子
    assert_eq!(users_filter.predicate, "row.birthday > '1990-01-01'");
    assert_eq!(users_filter.output, vec!["name", "email"]);
    
    // 验证 pipeline
    assert_eq!(pipeline.operators.len(), 1);
    assert!(pipeline.parents.is_empty());
} 