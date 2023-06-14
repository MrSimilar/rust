# 5.使用结构体组织相关联的数据

## 5.1 定义并实例化结构体

```rust
//定义结构体，使用关键字struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//实例化结构体以及结构体的使用
fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    //获取结构体当中的值
    println!("user1.email{}",user1.email);
    
    //改变结构体当中的值
    user1.email = "xinyet@aliyun.com";
    
    
}
```

## 5.1.1 使用字段初始化简写语法 

```rust
struct User{
    active: bool,
    username: String,
    email: String,
    age: i32,
}
//当参数名称和字段名称相同的时候，可以省略赋值的过程
fn build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        active: true,
        age: 18,
    }
}

```

## 5.1.2 使用结构体更新语法从其他实例创建实例

```rust
struct User{
    address: String,
    username: String,
    password: String,
    email: String,
    age: i32,
}

fn main(){
    
    //创建了一个结构体实例
    let user1 = User{
        address: String::from("SCCD"),
        username: String::from("username1"),
        password: String::from("password1"),
        email: String::from("email1@qq.com"),
        age: 18,
    };
    
    //根据上面的结构体实例，创建一个新的结构体实例
    let user2 = User{
        address: user1.address, //当这个属性和user1这个结构体里面的内容相同时
        username: String::from("username2"),
        password: String::from("password2"),
        email: String::from("email2@qq.com"),
        age: 28,
    };
    
    
    //也可以写不同的属性内容，其他相同的部分通过 ..user1来简写
    // 假设两个实例只有age不同，其他都相同
    let user3 = User{
        age: 32,
        ..user2
    };
}
```

    请注意，结构更新语法就像带有 = 的赋值，因为它移动了数据，新创建的实例可以使用，旧的实例因为移动的存在，就不能再使用了；
    如果，移动的数据是标量类型，那么不会影响之前实例的使用；

## 5.1.3 使用没有命名字段的元组结构体来创建不同的类型

```rust

//元组结构体的定义
struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

fn main() {
    //实例化
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

## 5.1.4 没有任何字段的类单元结构体

```rust
// 我们也可以定义一个没有任何字段的结构体！它们被称为 类单元结构体（unit−like structs）因为它们类
//似于 ()，即” 元组类型” 一节中提到的 unit 类型。
// 类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用。
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

## 5.1.5 结构体数据的所有权
```rust
// 以下程序编译会报错，需要加上生命周期，因为属性存储的是引用。但是现在我们可以通过String类型来解决这个问题
struct User {
    active: bool,
    username: &str, //引用
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
```


# 5.2 一个使用结构体的示例程序
    让我们编写一个计算长方形面积的程序。具体程序，参见main.rs


# 5.3 方法语法（方法必须是实例调用， 函数的调用不需要实例）
    方法（method）与函数类似：它们使用 fn 关键字和名称声明，可以拥有参数和返回值，同时包含在某
    处调用该方法时会执行的代码。不过方法与函数是不同的，因为它们在结构体的上下文中被定义（或者
    是枚举或 trait 对象的上下文，将分别在第六章和第十七章讲解），并且它们第一个参数总是 self，它代
    表调用该方法的结构体实例。
## 5.3.1 定义方法