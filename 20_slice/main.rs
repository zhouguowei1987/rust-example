fn main(){
    let mut v = Vec::new();
    v.push("Go语言极简一本通");
    v.push("Go语言微服务架构核心22讲");
    v.push("从0到Go语言微服务架构师");

    println!("v.len = {}", v.len());
    // let 切片值 = &变量[起始位置..结束位置]
    // [起始位置..结束位置]，这是一个左闭右开的区间。
    // 起始位置最小值是0。
    // 结束位置是数组、向量、字符串的长度。
    let s1 = &v[1..3];
    println!("s1 = {:?}", s1);

    show_slice(s1);

    let mut v2 = Vec::new();
    v2.push("Go语言极简一本通");
    v2.push("Go语言微服务架构核心22讲");
    v2.push("从0到Go语言微服务架构师");
    println!("modify_slice之前：{:?}", v2);

    modify_slice(&mut v2[1..3]);
    println!("modify_slice之后：{:?}", v2);
}

fn show_slice(s:&[&str]){
    println!("show_slice函数内：{:?}", s)
}

fn modify_slice(s: &mut [&str]){
    s[0] = "更改值：哈哈哈";
    println!("modify_slice : {:?}", s);
}