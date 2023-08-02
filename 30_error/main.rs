fn main(){
    // panic!("出错啦");
    println!("Hello Rust");

    // let v = vec!["Go语言极简一本通","Go语言微服务架构核心22讲","从0到Go语言微服务架构师"];
    // println!("{:?}", v[5])

    let f = std::fs::File::open("abc,jpg").expect("无法打开该文件");
    println!("{:?}", f);

    let result = is_even(6).unwrap();
    println!("result：{}", result);

    let result1 = is_even(11).unwrap();
    println!("result1：{}", result1);

}

fn is_even(no:i32) -> Result<bool, String>{
    return if no % 2 == 0{
        Ok(true)
    }else{
        Err("输入值，不是偶数".to_string())
    }
}