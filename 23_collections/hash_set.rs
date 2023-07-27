use std::collections::HashSet;

fn main(){
    let mut study_set = HashSet::new();
    study_set.insert("Go语言极简一本通");
    study_set.insert("Go语言微服务核心架构22讲");
    study_set.insert("从0到Go语言微服务架构师");

    println!("study_set : {:?}", study_set);
    println!("study_set.len : {}", study_set.len());

    study_set.insert("从0到Go语言微服务架构师");
    println!("study_set : {:?}", study_set);

    for item in study_set.iter(){
        println!("item: {}", item);
    }

    match study_set.get(&"从0到Go语言微服务架构师") {
        Some(val) => {
            println!("找到了 : {}", val);
        },
        None => {
            println!("未找到");
        }
    }

    if study_set.contains(&"从0到Go语言微服务架构师"){
        println!("包含");
    }else {
        println!("不包含");
    }

    // remove() 方法用于从集合中删除指定的值。
    // 如果该值在集合中，则返回 true，如果不存在则返回 false
    let x = study_set.remove(&"Go语言极简一本通");
    println!("x = {:?}", x);
    println!("study_set = {:?}", study_set);
}