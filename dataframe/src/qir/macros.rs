use crate::qir::{Column, DataType, Table};
use std::rc::Rc;

/// Macro for creating a table
/// 
/// # Example
/// 
/// ```rust
///     table!{
///         name: "users",
///         columns: [
///             column! { name = "id", data_type = I64 },
///             column! { name = "name", data_type = String },
///         ],
///     }
/// }
/// ```
/// 
#[macro_export]
macro_rules! column {
    { name = $x:expr, data_type = $type:ident } => {
        Column {
            name: $x.to_string(),
            data_type: DataType::$type,
        }
    }
}

#[macro_export] 
macro_rules! table {
    {
        name: $name:expr,
        columns: [ $($column:expr),* $(,)? ],
    } => {
        Table {
            name: $name.to_string(),
            columns: vec![ $($column),* ],
        }
    }
}
/// 宏用于创建 Scan 算子
/// 
/// # 示例
/// 
/// ```rust
/// scan! {
///     name: "users_scan",
///     table: users_table,
///     output: ["id", "name", "age"]
/// }
/// ```
#[macro_export]
macro_rules! scan {
    {
        name: $name:expr,
        table: $table:expr,
        output: [ $($field:expr),* $(,)? ]
    } => {
        Scan {
            name: $name.to_string(),
            table: $table,
            output: vec![ $($field.to_string()),* ]
        }
    }
}

/// 宏用于创建 Filter 算子
/// 
/// # 示例
/// 
/// ```rust
/// filter! {
///     input: scan_op,
///     predicate: |row| row.age > 18,
///     output: ["id", "name", "age"]
/// }
/// ```
#[macro_export]
macro_rules! filter {
    {
        input: $input:expr,
        predicate: $predicate:expr,
        output: [ $($field:expr),* $(,)? ]
    } => {
        Filter {
            input: $input,
            predicate: $predicate.to_string(),
            output: vec![ $($field.to_string()),* ]
        }
    }
}
/// 宏用于创建 IdentitySink 算子
/// 
/// # 示例
/// 
/// ```rust
/// identity! {
///     input: agg_op
/// }
/// ```
#[macro_export]
macro_rules! identity {
    {
        input: $input:expr
    } => {
        Rc::new(IdentitySink {
            input: $input
        })
    }
}

/// 宏用于创建 pipeline
/// 
/// # 示例
/// 
/// ```rust
/// pipeline! {
///     source: scan_op,
///     operators: [filter_op, join_op],
///     sink: agg_op,
///     parent: pipeline1
/// }
/// ```
#[macro_export]
macro_rules! pipeline {
    {
        source: $source:expr,
        operators: [ $($operator:expr),* $(,)? ],
        sink: $sink:expr
        $(, parent: $parent:expr)?
    } => {
        Pipeline {
            source: $source,
            operators: vec![ $($operator),* ],
            sink: $sink,
            parents: vec![]
        }
    }
}


