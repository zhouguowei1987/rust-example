fn main(){
    // 使用 const 关键字定义常量。
    // 定义常量时必须指定数据类型。
    // 常量名称的命名规则和之前变量的命名规则一样，但常量名称一般都是大写字母。
    // const PI: f64 = 3.1415926;
    // println!("PI is {}", PI);

    // let name ="《Go语言极简一本通》";
    // let name="《从0到Go语言微服务架构师》";
    // println!("name is {}", name)

    // let price = 199;
    // let price  = "299";
    // println!("price is {}", price);

    // const DISCOUNT:f64 = 0.8;
    // const DISCOUNT:f64 = 0.6;
    // `DISCOUNT` must be defined only once in the value namespace of this block

    static BOOK: &str = "《Go语言极简一本通》";
    println!("BOOK is {}", BOOK);

    static AA:isize = 1;
    println!("AA is {}", AA);

}