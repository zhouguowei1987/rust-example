fn main(){
    let study_list = vec!["Go语言极简一本通","Go语言微服务架构核心22讲","从0到Go语言微服务架构师"];

    // 如果我们要在Borrowing（借用）的时候改变其中的值：
    // 变量要用mut关键字。
    // 函数参数为可变的要用 &mut 关键字。
    // 传递参数的时候，也要用 &mut 关键字。
    let mut study_list2 = study_list;
    show2(&mut study_list2);
    println!("study_list2 : {:?}", study_list2);
}

fn show2(v:&mut Vec<&str>){
    let modify_v = "更改值：哈哈哈";
    v[0] = modify_v;
    println!("v : {:?}", v);
}