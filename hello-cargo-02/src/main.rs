
//1. Cargo 是 Rust 的构建系统和包管理器
//2.  cargo --version 查看其版本 //cargo 1.72.0-nightly (b0fa79679 2023-06-03)
//3. 该项目的结构就是一个普通的cargo创建的样子
//4. cargo build 编译生成二进制文件，在于src同级目录下的target/debug下面
//5. cargo run   编译并运行生成的可执行文件
//6. cargo check 该命令快速检查代码确保其可以编译，但并不产生可执行文件
//7. 可以使用 cargo build −−release 来优化编译项目,主要用于发布项目。这会在 target∕release 而不是 target∕debug 下生成可执行文件。
fn main() {
    println!("Hello, cargo!");
}
