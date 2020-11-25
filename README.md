批量把 GBK 编码的文件转成 UTF8，需要在命令提示符中运行，PowerShell 的管道不支持中文的文件名。

```
cargo install recurse
recurse walk DIRECTORY_PATH_TO_CONVERT | cargo run --release gb2312
```
