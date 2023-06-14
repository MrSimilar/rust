fn main() {
    println!("Hello, world!");

    //一个使用结构体的示例程序,计算长方形面积的程序
    println!("{}", area(30, 50));

    //结构体调用
    let rectangle = Rectangle{
        width: 34,
        height: 32,
    };
    println!("{:?}", rectangle);
    println!("{:#?}", rectangle);
}

// 传入两个参数来计算长方形的面积
fn area(width: u32, height: u32) -> u32 {
    width * height
}

// 使用元组重构来计算长方形的面积
fn area1(dimensions: (u32,u32)) -> u32{
    dimensions.0 * dimensions.1
}

#[derive(Debug)]//外部属性，派生Debug trait
struct Rectangle{
    width: u32,
    height: u32,
}

//使用结构体重构：赋予更多意义
fn area2(rectangle: Rectangle) -> u32{
    rectangle.width * rectangle.height
}