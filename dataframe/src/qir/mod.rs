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
    pub name: String,
    pub columns: Vec<Column>,
}

/// Column definition for a column in a table
pub struct Column {
    pub name: String,
    pub data_type: DataType,
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
    pub name: String,
    pub table: Rc<Table>,
    pub output: Vec<String>     // TODO resolve symbol -> definition
}

impl Operator for Scan { }
impl Source for Scan { }

/// a Filter operator
pub struct Filter {
    pub input: Rc<dyn Operator>,
    pub predicate: String,  // TODO
    pub output: Vec<String>
}
impl Operator for Filter { }

pub struct IdentitySink {
    pub input: Rc<dyn Operator>,
}

impl Operator for IdentitySink {}
impl Sink for IdentitySink {}

pub struct Pipeline {
    pub source: Rc<dyn Source>,
    pub operators: Vec<Rc<dyn Operator>>,
    pub sink: Rc<dyn Sink>,
    pub parents: Vec<Rc<Pipeline>>,
}

pub struct Topology {
    pub main: Rc<Pipeline>,
}

