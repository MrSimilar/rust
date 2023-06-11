fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5]; //hello 前闭后开
    let world = &s[6..11]; // world 前闭后开

    println!("{}", hello);
    println!("{}", world);
    println!("{}", s);

    let s = "Hello, world!";//字符串字面值就是 slice
}
