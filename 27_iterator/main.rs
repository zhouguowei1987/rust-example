fn main(){
    let v = vec![
        "Go语言极简一本通",
        "Go语言微服务架构核心22讲",
        "从0到Go语言微服务架构师"
    ];

    // Iterator 特质有两个函数：
    // 一个是 iter()，用于返回一个 迭代器 对象，也称之为 项 ( items ) 。
    // 一个是 next()，用于返回迭代器中的下一个元素。如果已经迭代到集合的末尾（最后一个项后面）则返回 None。
    let mut it = v.iter();
    println!("item1 : {:?}", it.next());
    println!("item2 : {:?}", it.next());
    println!("item3 : {:?}", it.next());
    println!("item4 : {:?}", it.next());

    // 返回一个只读可重入迭代器，迭代器元素的类型为 &T
    let iter = v.iter();

    // // 返回一个只读不可重入迭代器，迭代器元素的类型为 T
    // let iter = v.into_iter();

    // // 返回一个可修改可重入迭代器，迭代器元素的类型为 &mut T
    // let iter = v.iter_mut();
    let mut i:u8 = 1;
    for item in iter{
        println!("item{i} : {:?}", item);
        i = i + 1;
    };
    println!("v : {:?}", v)
}