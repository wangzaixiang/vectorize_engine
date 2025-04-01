# 练习 arrow 的项目

1. 理解 arrow spec: https://arrow.apache.org/docs/format/Columnar.html
   - IPC file format: 
     - write users.arrow file: poc/try_arrow/src/main.rs#test2 
     - read users.arrow file using FileReader: not zero copy version: poc/try_arrow/src/main.rs#test3
     - read users.arrow file using zero copy version: poc/try_arrow/src/main.rs#test5
     - 对照理解 spec/{File, Message, Schema}.fbs 理解 arrow 的文件格式，通过单步调试 test5 的代码来理解 arrow 的文件格式
2. 理解 arrow 的核心数据结构
   - 理解 arrow 的数据结构，通过单步调试 test5 的代码来理解 arrow 的数据结构


## Questions
1. how to debug flatbuffers code? 
2. how to debug &dyn Array?