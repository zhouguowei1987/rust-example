use std::collections::HashMap;

fn main(){
    let mut process = HashMap::new();
    process.insert("Go语言极简一本通", 1);
    process.insert("Go语言微服务核心架构22讲", 2);
    process.insert("从0到Go语言微服务架构师", 3);

    println!("process : {:?}", process);
    println!("process.length : {}", process.len());

    match process.get(&"从0到Go语言微服务架构师") {
        Some(v) => {
            println!("HashMap v : {}", v);
        },
        None => {
            println!("未找到！");
        }
    }

    for (k, v) in process.iter(){
        println!("k : {}, v : {}", k, v);
    }

    if process.contains_key(&"Go语言极简一本通"){
        println!("包含");
    }else {
        println!("不包含")
    }

    // remove() 用于从哈希表中删除指定的键值对。
    // 如果键值对存在则返回删除的键值对，返回的数据格式为 (&'a K, &'a V)。
    // 如果键值对不存在则返回 None
    let x = process.remove(&"Go语言极简一本通");
    println!("x = {:?}", x);
    println!("process = {:?}", process);
}