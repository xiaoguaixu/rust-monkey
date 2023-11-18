参考《用Go语言自制解释器》，学习和使用 rust 编程语言 编写的解释器
代码目录和结构和《用Go语言自制解释器》一致，这本书上的功能在这里也都
实现，有详细的测试代码。

ast 两种实现方式，一种和《用Go语言自制解释器》类似，用trait来表示ast。代码在 master branch上
另一种是用 enum表示ast的，代码在 ast_enum branch上。