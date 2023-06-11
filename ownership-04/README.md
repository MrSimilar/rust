# 所有权
## 1.所有权定义
    通过所有权系统管理内存，编译器在编译时会根据一系列的规则进行检查。如果违反了任何这些规
    则，程序都不能编译。在运行时，所有权系统的任何功能都不会减慢程序。
## 2.栈与堆（内存）
    栈：先进后出
    堆：通过指针指向堆中的某块区域
## 3.所有权规则
* rust中的每一个值都有一个被称为其所有者的变量；
* 值在任何时刻有且只有一个所有者；
* 当所有者（变量）离开作用域，这个值将被丢弃；

## 4.变量作用域
```rust
 fn main() {
    { // s 在这里无效, 它尚未声明
        let s = "hello"; // 从此处起，s 是有效的
        // 使用 s
    } // 此作用域已结束，s 不再有效，rust默认调用drop函数销毁String
}
```

## 5.String类型（通过String类型来认识所有权）
```rust
fn main() {
    let s = String::from("hello"); //String 变量的定义
}
```
## 6.内存的分配
* 标量类型、数组、元组等知道大小的数据类型分配在栈空间；
* 复杂类型分配在堆空间，还在栈空间分配其指针，长度，容量等；
* 复杂类型的变量值的变更，称之为移动。移动过后之前变量的所有权失效，数据被释放；
* rust中，对复杂类型的拷贝一般为浅拷贝，只有调用clone（）方法，才能实现深拷贝；
* 标量类型值的更替，是直接复制栈空间的数据，不存在浅拷贝一说；

## 7.引用和借用

针对于rust的内存管理，以及所有权的概念。会出现一个问题，导致变量的所有权总是转过去转过来，相对麻烦。
因而，可以用引用来简化编程；

### 7.1 引用和借用

```rust
fn main() {
    let s1 = String::from("hello"); // 定义了一个String类型的变量s1
    let len = calculate_length(&s1); // 将s1的引用传入到函数中  &s1即为s1的引用，这种行为称为借用
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()  // 计算传入引用的长度，并返回
}
```

引用和变量一样，也分为可变引用和不可变引用

注意一个引用的作用域从声明的地方开始一直持续到最后一次使用为止。
```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用
    let r3 = &mut s; // 没问题
    println!("{}", r3)
}
```

### 7.2 悬垂引用
悬垂指针是其指向的内存可能已经被分配给其它持有者

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle 返回一个字符串的引用
    let s = String::from("hello");// s 是一个新字符串
    &s    // 返回字符串 s 的引用
}  // 这里 s 离开作用域并被丢弃。其内存被释放。
```

解决方案
```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
```

### 7.3 引用的规则

* 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用。
* 引用必须总是有效的。

## 8.slice类型

字符串slice是String中一部分值的引用，它看起来像这样
```rust
fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5]; //hello 前闭后开
    let world = &s[6..11]; // world 前闭后开
}
```

## 9.总结
    所有权、借用和 slice 这些概念让 Rust 程序在编译时确保内存安全。Rust 语言提供了跟其他系统编程语
    言相同的方式来控制你使用的内存，但拥有数据所有者在离开作用域后自动清除其数据的功能意味着你
    无须额外编写和调试相关的控制代码。
    所有权系统影响了 Rust 中很多其他部分的工作方式，所以我们还会继续讲到这些概念，这将贯穿本书的
    余下内容。