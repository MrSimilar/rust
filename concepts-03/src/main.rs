fn main() {
    //-----------变量的不可变性 start -------------------
    // let x = 5;
    // println!("The value of x is: {}", x);
    // x = 6; //cannot assign twice to immutable variable
    // println!("The value of x is: {}", x);
    //-----------变量的不可变性 enf -------------------

    //-----------变量的可变性 start-------------------
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    //-----------变量的可变性 end-------------------

    //-----------常量的定义 start-------------------
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; //整个程序生命周期有效
    //-----------常量的定义 end-------------------

    //-----------变量的隐藏 start-------------------
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);    //12
    }
    println!("The value of x is: {}", x);  // 6
    //-----------变量的隐藏 end-------------------


    // ------------------- 元组 start -----------------------------
    let tup: (i32, f64, u8) = (500, 6.4, 1);  //(i32, f64, u8) 类型注解

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;  //解构
    println!("The value of y is: {}", y);


    let x: (i32, f64, u8) = (500, 6.4, 1);  // 索引访问
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    // ------------------- 元组 end -----------------------------

    // ------------------- 数组 start -----------------------------
    //数组申明：
    let array1: [i32; 4] = [1, 2, 3, 4]; //i32 指定数组元素的类型，4指定数组的长度

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    let array2 = [1; 5]; // means let array2 = [1,1,1,1,1]; 1表示默认值，5表示数组长度

    // 数组访问：
    println!("{}", array1[0]); //访问array1数组的第一个元素，数组索引从0开始
    for item in 0..array1.len(){
        println!("{}", array1[item]);
    }
    // ------------------- 数组 end -----------------------------


    // ------------------- 控制流 start -----------------------------
    let number = 13;
    if number >4 {
        println!("{}", number);
    }else {
        println!("{}", number);
    }

    let condition = true;
    let number = if condition {5} else { 6 };


    // ------------------- 控制流 end -----------------------------


    //作业1 相互转换摄氏与华氏温度
    println!("{}", centigrade_to_fahrenheit(32 as f64)); //89.6

}

//centigrade to fahrenheit
fn centigrade_to_fahrenheit(x: f64) -> f64{
    x*9.0/5.0+32.0
}


// 函数的定义
fn another_function(x: i32) -> i32{
    x+1
}
