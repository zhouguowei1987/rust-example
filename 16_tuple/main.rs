fn main(){
    // 元组一旦定义，就不能再增长或缩小，长度是固定的。元组的下标从 0 开始
    let t:(&str, &str) = ("Go语言极简一本通", "掌握Go语言语法，并且可以完成单体服务应用");
    println!("{:?}", t);

    println!("t.0 = {}", t.0);
    println!("t.1 = {}", t.1);

    show_tuple(t);

    let (book, target) = t;
    println!("book = {}", book);
    println!("target = {}", target);
}

fn show_tuple(tuple:(&str, &str)){
    println!("{:?}", tuple)
}