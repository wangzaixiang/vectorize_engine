use arrow::datatypes::DataType;

trait PrimaryArrayType {
    const DATA_TYPE: arrow::datatypes::DataType;
}

macro_rules! impl_primary_type {
    ($t:ty, $dt:expr) => {
        impl PrimaryArrayType for $t {
            const DATA_TYPE: arrow::datatypes::DataType = $dt;
        }
    };
}

impl_primary_type!(bool, arrow::datatypes::DataType::Boolean);
impl_primary_type!(i8, arrow::datatypes::DataType::Int8);
impl_primary_type!(i16, arrow::datatypes::DataType::Int16);
impl_primary_type!(i32, arrow::datatypes::DataType::Int32);
impl_primary_type!(i64, arrow::datatypes::DataType::Int64);
impl_primary_type!(u8, arrow::datatypes::DataType::UInt8);
impl_primary_type!(u16, arrow::datatypes::DataType::UInt16);
impl_primary_type!(u32, arrow::datatypes::DataType::UInt32);
impl_primary_type!(u64, arrow::datatypes::DataType::UInt64);
impl_primary_type!(f32, arrow::datatypes::DataType::Float32);
impl_primary_type!(f64, arrow::datatypes::DataType::Float64);

/// a better way to define a PrimaryArray than arrow::array::PrimitiveArray
struct PrimaryArray<T: PrimaryArrayType> {
    data_type: DataType,
    data: Vec<T>,
}

impl <T: PrimaryArrayType> PrimaryArray<T> {
    fn new(data: Vec<T>) -> Self {
        Self {
            data_type: T::DATA_TYPE, // the same as data_type: <T as PrimaryArrayType>::DATA_TYPE,
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary_type() {
        // better than arrow::array::PrimitiveArray
        // let i32_arr: PrimaryArray<Int32Type> = arrow::array::Int32Array::from(vec![1, 2, 3]);
        let i32_arr: PrimaryArray<i32> = PrimaryArray::new(vec![1, 2, 3] );
        assert_eq!(i32_arr.data_type, DataType::Int32);
    }
}