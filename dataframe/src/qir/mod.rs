use std::rc::Rc;

pub mod macros;

pub trait Operator {

}
pub trait Source: Operator {

}
pub trait Sink: Operator {

}

/// Type definitions for a table
pub struct Table {
    name: String,
    columns: Vec<Column>,
}

/// Column definition for a column in a table
pub struct Column {
    name: String,
    data_type: DataType,
}

/// Data type for columns
pub enum DataType {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Decimal,
    Bool,
    String,     // TODO Char(N) or VarChar(N)
    Date,
    DateTime,
    List(Box<DataType>),
    Struct(Box<Table>),
    Map(Box<DataType>, Box<DataType>),
}

/// a Scan source operator
pub struct Scan {
    name: String,
    table: Rc<Table>,
    output: Vec<String>     // TODO resolve symbol -> definition
}

impl Operator for Scan { }
impl Source for Scan { }

/// a Filter operator
pub struct Filter {
    input: Rc<dyn Operator>,
    predicate: String,  // TODO
    output: Vec<String>
}
impl Operator for Filter { }

pub struct IdentitySink {
    input: Rc<dyn Operator>,
}

impl Operator for IdentitySink {}
impl Sink for IdentitySink {}

pub struct Pipeline {
    source: Rc<dyn Source>,
    operators: Vec<Rc<dyn Operator>>,
    sink: Rc<dyn Sink>,
    parents: Vec<Rc<Pipeline>>,
}

pub struct Topology {
    main: Rc<Pipeline>,
}

#[cfg(test)]
mod tests {
    mod example1_test;
}