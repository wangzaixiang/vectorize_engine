pub mod datatype;
pub mod qir;

#[macro_use]
pub use qir::macros::*;

// pub trait Vector<T> {
//
//     fn len(&self) -> usize;
//
//     fn get(&self, index: usize) -> T;
//
//     /// load N (2/4/8/16/32/64) elements from the vector starting from the given index
//     fn load<const N: usize>(&self, from: usize) -> [T; N];
//
//     fn dataType(&self) -> DataType;
//
//     fn filter(&self, predicate: fn(&T) -> bool) -> Self;
// }
//
// enum DataType {
//     Bool,
//     Int8,
//     Int16,
//     Int32,
//     Int64,
//     UInt8,
//     UInt16,
//     UInt32,
//     UInt64,
//     Float32,
//     Float64,
//     BigDecimal,
//     String,
//     Date,
//     DateTime,
//     List(Box<DataType>),
//     Struct(Vec<String>, Vec<DataType>),
//     Map(Box<DataType>, Box<DataType>),
// }
//
// pub trait VectorBuilder<VT, T> where VT: Vector<T> {
//
//     fn push(&mut self, value: T);
//
//     fn build(&self) -> VT;
// }
//
// /// TODO: a DataFrame is a Vector<Struct>
// pub trait DataFrame {
//
//     fn width(&self) -> usize;
//
//     fn len(&self) -> usize;
//
//     fn series<T>(&self, col_index: usize) -> &dyn Vector<T>;
//
//     fn get<T>(&self, col_index: usize, row_index: usize) -> T;
//
//     fn load<T, const N: usize>(&self, col_index: usize, from: usize) -> [T; N];
//
// }