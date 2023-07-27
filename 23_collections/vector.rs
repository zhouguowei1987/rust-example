fn main(){
    let mut v  = Vec::new();
    v.push("Go语言极简一本通");
    v.push("Go语言微服务核心架构22讲");
    v.push("从0到Go语言微服务架构师");

    println!("v : {:?}", v);
    println!("v.len = {}", v.len());

    let mut v2 = vec![
        "Go语言极简一本通",
        "Go语言微服务核心架构22讲",
        "从0到Go语言微服务架构师"
    ];
    println!("v2 : {:?}", v2);

    let x = v2.remove(0);
    println!("x : {}", x);
    println!("v2 : {:?}", v2);

    if v.contains(&"Go语言极简一本通"){
        println!("包含");
    }else {
        println!("不包含");
    }

    let y = v[0];
    println!("y = {}", y);

    for item in v{
        println!("item = {}", item)
    }

}