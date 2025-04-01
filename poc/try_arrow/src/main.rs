use std::any::Any;
use arrow::array::{Int32Array, StringArray, Date32Array, Array};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use arrow::ipc::writer::FileWriter;
use arrow::ipc::reader::{read_footer_length, FileDecoder, FileReader};
use std::fs::File;
use std::sync::Arc;
use std::io::Cursor;
use std::ptr;
use arrow::buffer::Buffer;
use arrow::ipc::{root_as_footer, Block};
use arrow::ipc::convert::fb_to_schema;
use arrow::error::Result;
use arrow_cast::pretty::{pretty_format_batches, pretty_format_columns};

fn main() {
    let i32s = Int32Array::from(vec![1, 2, 3, 4, 5]);
    if i32s.len() > 0 {
        println!("i32s[0]: {:?}", i32s.value(0));
    }
    // for i in 0..i32s.len() {
    //     println!("i32s[{}]: {:?}", i, i32s.value(i));
    // }
    println!("{:?}", i32s);
}

#[test]
fn test1(){
    let i32s = Int32Array::from(vec![1, 2, 3, 4, 5]);
    if i32s.len() > 0 {
        println!("i32s[0]: {:?}", i32s.value(0));
    }
    println!("{:?}", i32s);

    let arr2 = Int32Array::from(vec![Some(1), None, Some(3), None, Some(5)]);
    println!("{:?}", arr2);
}

// table users(id: int, name: string, birthday: date， email: string)
// 包括如下测试数据：(1, "Alice", "2000-01-01", "alice@example.com"), (2, "Bob", "2000-01-02", "bob@example.com"), (3, "Charlie", "2000-01-03", null)
// 生成一个 arrow ipc file: users.arrow 文件，包括上述的内容
#[test]
fn test2(){
    let id = Arc::new(Int32Array::from(vec![0x31, 0x32, 0x33]));
    let name = Arc::new(StringArray::from(vec!["Alice", "Bob", "Charlie"]));
    let birthday = Arc::new( Date32Array::from(vec![10957, 10958, 10959]));  // TODO 日期的处理
    let email = Arc::new(StringArray::from(vec![Some("alice@example.com"), Some("bob@example.com"), None]));

    let schema = Schema::new(vec![
        Field::new("id", DataType::Int32, false),
        Field::new("name", DataType::Utf8, false),
        Field::new("birthday", DataType::Date32, false),
        Field::new("email", DataType::Utf8, true),
    ]);

    let batch = RecordBatch::try_new(Arc::new(schema.clone()), vec![id, name, birthday, email]).unwrap();

    let file = File::create("users.arrow").unwrap();
    let mut writer = FileWriter::try_new(file, &schema).unwrap();
    writer.write(&batch).unwrap();
    writer.finish().unwrap();
}

// 读取 users.arrow 文件，并打印其内容
#[test]
fn test3(){

    let file = File::open("users.arrow").unwrap();
    let reader = FileReader::try_new(file, None).unwrap();
    println!("{:?}", reader);
    let schema = reader.schema();
    let batches = reader.num_batches();
    println!("{:?}", schema);
    println!("{:?}", batches);

    for batch in reader {
        println!("{:?}", batch.unwrap());
    }
}

/// 读取 users.arrow 文件，并打印其内容
/// 验证 arrow 是否支持 mmap 方式读取文件，从而避免重新分配缓冲区，目前，来看，似乎并没有达到这个效果
#[test]
fn test4() {
    use memmap2::Mmap;
    let file = File::open("users.arrow").unwrap();
    let mmap = unsafe { Mmap::map(&file).unwrap() };
    let cursor = Cursor::new(&mmap[..]);
    let reader = FileReader::try_new(cursor, None).unwrap();

    // 打印 schema 信息
    println!("Schema: {:?}", reader.schema());

    // 遍历所有批次并打印内容
    for (i, batch) in reader.enumerate() {
        let batch = batch.unwrap();
        println!("\nBatch {}:", i);
        println!("  id: {:?}", batch.column(0));
        println!("  name: {:?}", batch.column(1));
        println!("  birthday: {:?}", batch.column(2));
        println!("  email: {:?}", batch.column(3));

        let col0 = batch.column(0);  // TODO how to safe downcast?
        let c0l0_down = col0.as_any().downcast_ref::<Int32Array>().unwrap();
        println!("{:?}", c0l0_down);
    }
}

#[test]
fn test5() {
    // 使用 zero-copy 方式读取文件
    let file = File::open("users.arrow").unwrap();
    let mmap = unsafe { memmap2::Mmap::map(&file).expect("failed to mmap file") };
    let bytes = bytes::Bytes::from_owner(mmap);
    let buffer = Buffer::from(bytes);

    // Now, use the FileDecoder API (wrapped by `IPCBufferDecoder` for
    // convenience) to crate Arrays re-using the data in the underlying buffer
    let decoder = IPCBufferDecoder::new(buffer);
    assert_eq!(decoder.num_batches(), 1);

    // Create the Arrays and print them
    for i in 0..decoder.num_batches() {
        let batch = decoder.get_batch(i).unwrap().expect("failed to read batch");
        let col0 = batch.column(0);
        let x = col0.as_any();
        // get ptr of x
        let x_ptr = x as *const dyn Any as *const u8;
        let x_ptr2: &Int32Array = unsafe { &*(x_ptr as *const Int32Array) };
        let col1 = batch.column(0).as_any().downcast_ref::<Int32Array>().unwrap();
        let col2 = batch.column(1).as_any().downcast_ref::<StringArray>().unwrap();
        println!("{:?}", col1);
        println!("{:?}", col2);
        assert_eq!(3, batch.num_rows());
        println!("Batch {i}\n{}", pretty_format_batches(&[batch]).unwrap());
    }
}

struct IPCBufferDecoder {
    /// Memory (or memory mapped) Buffer with the data
    buffer: Buffer,
    /// Decoder that reads Arrays that refers to the underlying buffers
    decoder: FileDecoder,
    /// Location of the batches within the buffer
    batches: Vec<Block>,
}

impl IPCBufferDecoder {
    fn new(buffer: Buffer) -> Self {
        let trailer_start = buffer.len() - 10;
        let footer_len = read_footer_length(buffer[trailer_start..].try_into().unwrap()).unwrap();
        let footer = root_as_footer(&buffer[trailer_start - footer_len..trailer_start]).unwrap();

        let schema = fb_to_schema(footer.schema().unwrap());

        let mut decoder = FileDecoder::new(Arc::new(schema), footer.version());

        // Read dictionaries
        for block in footer.dictionaries().iter().flatten() {
            let block_len = block.bodyLength() as usize + block.metaDataLength() as usize;
            let data = buffer.slice_with_length(block.offset() as _, block_len);
            decoder.read_dictionary(block, &data).unwrap();
        }

        // convert to Vec from the flatbuffers Vector to avoid having a direct dependency on flatbuffers
        let batches = footer
            .recordBatches()
            .map(|b| b.iter().copied().collect() )
            .unwrap_or_default();

        Self {
            buffer,
            decoder,
            batches,
        }
    }

    /// Return the number of [`RecordBatch`]es in this buffer
    fn num_batches(&self) -> usize {
        self.batches.len()
    }

    /// Return the [`RecordBatch`] at message index `i`.
    ///
    /// This may return `None` if the IPC message was None
    fn get_batch(&self, i: usize) -> Result<Option<RecordBatch>> {
        let block = &self.batches[i];
        let block_len = block.bodyLength() as usize + block.metaDataLength() as usize;
        let data = self
            .buffer
            .slice_with_length(block.offset() as _, block_len);
        self.decoder.read_record_batch(block, &data)
    }
}
