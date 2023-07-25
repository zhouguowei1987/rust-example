fn main(){
    let lesson:&str = "《Go语言极简一本通》";
    println!("lesson is {}", lesson);
    // lesson is 《Go语言极简一本通》

    let s1 = String::new();
    println!("s1:{}, s1=>len:{}", s1, s1.len());
    // s1:, s1=>len:0

    let s2 = String::from("语言极简一本通");
    println!("s2:{}, s2=>len:{}", s2, s2.len());
    // s2:语言极简一本通, s2=>len:21

    // 创建一个新的字符串对象
    let mut s3 = String::new();
    // 在字符串末尾追加字符串
    s3.push_str("Go语言极简一本通");
    println!("s3:{}", s3);

    // 在原字符上追加字符，而不是返回一个新的字符串
    s3.push('O');
    s3.push('K');
    println!("{}", s3);
    // Go语言极简一本通OK

    let s4 = String::from("Go语言极简一本通");
    // 指定字符串子串替换成另一个字符串
    let result = s4.replace("Go", "Rust");
    println!("{}", result);
    // Rust语言极简一本通

    let s5 = String::from("Go语言极简一本通");
    // 返回字符串中的 总字节数  一个汉字3个字节
    println!("s5=>len:{}",s5.len());
    // s5=>len:23

    // 将字符串转换为字符串对象，方便以后可以有更多的操作
    let s6 = "从0到Go语言微服务架构师".to_string();
    println!("{}", s6);

    // 返回一个字符串对象的 字符串 字面量
    let s7 = String::from("Go语言微服务架构核心22讲");
    println!("{}", s7.as_str());

    let s8 = " \tGo语言极简一本通\tGo语言微服务架构核心22讲 \r\n从0到Go语言微服务架构师\r\n     ";
    println!("length is {}", s8.len());
    // 去除字符串头尾的空白符
    println!("length is {}", s8.trim().len());

    let s9 = "Go语言极简一本通、Go语言微服务架构核心22讲、从0到Go语言微服务架构师";
    // 将字符串根据某些指定的 字符串子串 分割，返回分割后的字符串子串组成的切片上的迭代器。
    for item in s9.split("、"){
        println!("{}", item);
    }

    let s10 = "从0到Go语言微服务架构师";
    // 将一个字符串打散为所有字符组成的数组
    for c in s10.chars(){
        println!("字符：{}", c)
    }

    let s11 = "Go语言极简一本通".to_string();
    let s12 = " 喜欢".to_string();
    // + 的内部实现是重写了 add() 方法
    // add(self,&str)->String {}
    let result = s11 + &s12;
    println!("s11 + s12 = {}", result)
}