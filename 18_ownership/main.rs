fn main(){
    // 栈
    // 它是一种 后进先出 的机制，类似我们日常的落盘子，只能一个一个向上方，然后从最上面拿一个盘子。
    // 一个变量要放到栈上，那么它的大小在编译时就要明确。i32 类型的变量，它就占用 4 个字节。
    // Rust 中可以放到栈上的数据类型，他们的大小都是固定的。
    // 如果是字符串，在运行时才会赋值的变量，在编译期的时候大小是未知或不确定的。所以字符串类型存储在堆上。

    // 堆
    // 用于编译时大小未知或不确定的，只有运行时才能确定的数据。
    // 在堆上存储一些动态类型的数据。堆是不受系统管理的，是用户自己管理的，也增加了内存溢出的风险。

    let a = 18;
    let b = a;
    println!("a = {}, b = {}", a, b);

    // v1 拥有堆上数据的所有权。（每次只能有一个变量对堆上数据有所有权）
    // v2=v1 v2 拥有了堆上数据的所有权。
    // v1 已经没有对数据的所有权了，所以再使用 v1 会报错。
    // 如果 Rust 检查到 2 个变量同时拥有堆上内存的所有权。会报错如上。
    // let v1 = vec!["Go语言极简一本通","Go语言微服务架构核心22讲","从0到Go语言微服务架构师"];
    // let v2 = v1;
    // println!("{:?}", v1);
    // move occurs because `v1` has type `Vec<&str>`, which does not implement the `Copy` trait

    // // study_list拥有堆上数据管理权
    // let study_list = vec!["Go语言极简一本通","Go语言微服务架构核心22讲","从0到Go语言微服务架构师"];
    // // study_list将所有权转移给了study_list2
    // let study_list2 = study_list;
    // // study_list2将所有权转让给参数v，study_list2不在可用
    // show(study_list2);
    // // study_list2已经不可用
    // println!("study_list2 : {:?}", study_list2);

    let study_list3 = vec!["Go语言极简一本通","Go语言微服务架构核心22讲","从0到Go语言微服务架构师"];
    let study_list4 = study_list3;
    let result = show2(study_list4);
    println!("result : {:?}", result);

    // 所有权只会发生在堆上分配的数据，基础数据类型(整型，浮点型，布尔，字符)存储在栈上，所以没有所有权的概念。
    // 基础类型可以认为是值拷贝，在内存上另外的地方，存储和复制来的数据，然后让新的变量指向它。
}

// fn show(v:Vec<&str>){
//     println!("v : {:?}", v);
// }

fn show2(v:Vec<&str>) -> Vec<&str>{
    println!("v : {:?}", v);
    return v;
}