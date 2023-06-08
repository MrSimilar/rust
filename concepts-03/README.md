## 编程的基本概念
# 1.关键字
    如下关键字目前有对应其描述的功能。

| 关键字        | 作用                                                           |
|------------|--------------------------------------------------------------|
| as         | 强制类型转换，消除特定包含项的 trait 的歧义，或者对 use 和 extern crate 语句中的项重命名    |
| async      | 返回一个 Future 而不是阻塞当前线程                                        |
| await      | 暂停执行直到 Future 的结果就绪                                          |
| break      | 立刻退出循环                                                       |
| const      | 定义常量或不变裸指针（constant raw pointer）                             |
| continue   | 继续进入下一次循环迭代                                                  |
| crate      | 链接（link）一个外部 crate 或一个代表宏定义的 crate 的宏变量                      |
| dyn        | 动态分发 trait 对象                                                |
| else       | 作为 if 和 if let 控制流结构的 fallback                               |
| enum       | 定义一个枚举                                                       |
| extern     | 链接一个外部 crate 、函数或变量                                          |
| false      | 布尔字面值 false                                                  |
| fn         | 定义一个函数或 函数指针类型 (function pointer type)                       |
| for        | 遍历一个迭代器或实现一个 trait 或者指定一个更高级的生命周期                            |
| if         | 基于条件表达式的结果分支                                                 |
| impl       | 实现自有或 trait 功能                                               |
| in         | for 循环语法的一部分                                                 |
| let        | 绑定一个变量                                                       |
| loop       | 无条件循环                                                        |
| match      | 模式匹配                                                         |
| mod        | 定义一个模块                                                       |
| move       | 使闭包获取其所捕获项的所有权                                               |
| mut        | 表示引用、裸指针或模式绑定的可变性                                            |
| pub        | 表示结构体字段、impl 块或模块的公有可见性                                      |
| ref        | 通过引用绑定                                                       |
| return     | 从函数中返回                                                       |
| Self       | 定义或实现 trait 的类型的类型别名                                         |
| self       | 表示方法本身或当前模块                                                  |
| static     | 表示全局变量或在整个程序执行期间保持其生命周期                                      |
| struct     | 定义一个结构体                                                      |
| super      | 表示当前模块的父模块                                                   |
| trait      | 定义一个 trait                                                   |
| true       | 布尔字面值 true                                                   |
| type       | 定义一个类型别名或关联类型                                                |
| union      | 定义一个 union 并且是 union 声明中唯一用到的关键字                             |
| unsafe     | 表示不安全的代码、函数、trait 或实现                                        |
| use        | 引入外部空间的符号                                                    |
| where      | 表示一个约束类型的从句                                                  |
| while      | 基于一个表达式的结果判断是否进行循环                                           |

# 2.变量和可变性
    变量默认是不可改变的（immutable）
    mut 关键字可以让变量可变 （mutable）
# 3.常量
    类似于不可变变量，常量 (constants) 是绑定到一个名称的不允许改变的值，不过常量与变量还是有一些区别。
# 4.隐藏
    定义一个与之前变量同名的新变量，那么该变量原有的值就被隐藏了
# 5.数据类型
## 5.1 标量类型
    标量（scalar）类型代表一个单独的值。Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型。
### 5.1.1 整型(i32)
| 长度      | 有符号(以补码形式存储) | 无符号   |
|---------|--------------|-------|
| 8-bit   | i8           | u8    |
| 16-bit  | i16          | u16   |
| 32-bit  | i32          | u32   | 
| 64-bit  | i64          | u64   |
| 128-bit | i128         | u128  |
| arch    | isize        | usize |

    整型字面值

| 数字字面值                    | 例子          |
|--------------------------|-------------|
| Decimal (十进制)            | 98_222      |
| Hex (十六进制)               | 0xff        |
| Octal (八进制)              | 0o77        |
| Binary (二进制)             | 0b1111_0000 |
| Byte (单字节字符)(仅限于u8)      | b'A'        |

### 5.1.2 浮点型(f64)
    Rust 也有两个原生的 浮点数（floating−point numbers）类型，它们是带小数点的数字。Rust 的浮点数
    类型是 f32 和 f64，分别占 32 位和 64 位。默认类型是 f64，因为在现代 CPU 中，它与 f32 速度几乎一
    样，不过精度更高。所有的浮点型都是有符号的。

### 5.1.3 布尔型
    正如其他大部分编程语言一样，Rust 中的布尔类型有两个可能的值：true 和 false。Rust 中的布尔类型使用 bool.

### 5.1.4 字符型
    Rust 的 char 类型是语言中最原生的字母类型。
    Rust 的 char 类型的大小为四个字节 (four bytes)，并代表了一个 Unicode 标量值（Unicode Scalar Value），这意味着它
    可以比 ASCII 表示更多内容


## 5.2 复合类型
    复合类型（Compound types）可以将多个值组合成一个类型。Rust 有两个原生的复合类型：元组（tuple）和数组（array）。

### 5.2.1 元组
    元组是一个将多个其他类型的值组合进一个复合类型的主要方式。
    1.元组长度固定：一旦声明，其长度不会增大或缩小。
    2.我们使用包含在圆括号中的逗号分隔的值列表来创建一个元组。
    3.元组中的每一个位置都有一个类型，而且这些不同值的类型也不必是相同的。

    4.没有任何值的元组 () 是一种特殊的类型，只有一个值，也写成 () 。该类型被称为 单元类型（unit type），
    而该值被称为 单元值（unit value）。如果表达式不返回任何其他值，则会隐式返回单元值。
### 5.2.2 数组
    另一个包含多个值的方式是 数组（array）。与元组不同，数组中的每个元素的类型必须相同。Rust 中的
    数组与一些其他语言中的数组不同，Rust 中的数组长度是固定的。
## 6.函数

### 6.1 函数定义
```rust
//定义： 
// 1. fn 定义函数的关键字
// 2. another_function 函数名称(snake case)
// 3. (x:i32) 函数参数体,必须指定类型；x称为parameter, 具体传入的值称为 argument
// 4. -> i32 定义函数的返回值类型是 i32
// 5. {} 函数体
fn another_function(x: i32) -> i32{
    x+1
}
```

### 6.1 函数参数
    1.在函数签名中，必须声明每个参数的类型。
    2.函数有多个参数的时候，用逗号分割
    3.函数的参数称为parameter, 具体传入的值称为 argument
### 6.2 语句和表达式
    语句（Statements）是执行一些操作但不返回值的指令。表达式（Expressions）计算并产生一个值。
## 7.注释 comments
```rust
// 这是一个简单的注释； 还有一个文档注释，之后再学习他
```

## 8.控制流
### 8.1 if表达式
    语法： if bool {} else if bool {} else {}
### 8.2 循环(loop while for)  break continue 同Java
#### 8.2.1 loop
```rust
fn main(){
    loop {
        println!("test"); //会一直打印test
    }
}
```
    循环标签的使用
```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
}
```

#### 8.2.2 while
```rust

fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}
```

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}
```

#### 8.2.3 for
```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

## 9. 作业
你做到了！这是一个大章节：你学习了变量、标量和复合数据类型、函数、注释、if 表达式和循环！如
果你想要实践本章讨论的概念，尝试构建如下程序：
• 相互转换摄氏与华氏温度。（摄氏度×9÷5＋32=华氏度）
• 生成 n 阶斐波那契数列。
• 打印圣诞颂歌 ”The Twelve Days of Christmas” 的歌词，并利用歌曲中的重复部分（编写循环）。
