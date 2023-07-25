fn main(){
    hello();
    println!("r1:{}", get_name());
    println!("r2:{}", get_name2());

    let price1 = 99;
    double_price(99);
    println!("外部的price是{}", price1);

    let mut price2 = 88;
    double_price2(&mut price2);
    println!("外部的price是{}", price2);

    let name:String = String::from("从0到Go语言微服务架构师");
    println!("调用show_name函数前:{}", name);
    show_name(name);
    // 对于复合类型，比如字符串，如果按照普通的方法传递给函数后，那么该变量将不可再访问
    // println!("调用show_name函数后:{}", name);
}
fn hello(){
    println!("Hello rust!");
}

fn get_name()->String{
    return String::from("Go语言微服务架构核心22讲")
}

fn get_name2()->String{
    String::from("从0到Go语言微服务架构师")
}

fn double_price(mut price1:i32){
    price1 = price1 * 2;
    println!("内部的price是{}", price1)
}

fn double_price2(price2: &mut i32){
    *price2 = *price2 * 2;
    println!("内部的price是{}", price2)
}

fn show_name(name:String){
    println!("name:{}", name)
}