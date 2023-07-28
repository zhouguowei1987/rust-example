use std::io::Write;

fn main(){
    let mut in_word = String::new();
    in_word.push_str("Hello ");
    in_word.push('O');
    in_word.push('K');
    in_word.push('!');
    println!("in_word : {}", in_word);

    println!("请输入....");
    // std::io::stdin() 返回标准输入流 stdin 的句柄。
    // read_line() 是标准入流 stdin 的句柄上的一个方法，
    // 从标准输入流读取一行的数据。返回值是一个 Result 枚举，
    // 而 unwrap() 则是一个帮助方法，用于简化可恢复错误的处理。
    // 它会返回 Result 中存储的实际值。
    // read_line() 方法会自动删除行尾的换行符 \n
    let result = std::io::stdin().read_line(&mut in_word).unwrap();
    println!("您的输入是：{}", in_word);
    println!("读取的字节数为：{}", result);

    // std::io::stdout() 会返回标准输出流 stdout 的句柄。\
    // write() 是标准输出流 stdout 的句柄上的一个方法，用于向标准输出流写入字节流内容。
    // write() 方法的返回值值一个 Result 枚举，而 unwrap() 则是一个帮助方法，用于简化可恢复错误的处理。它会返回 Result 中存储的实际值。
    // write() 方法并不会输出结束时自动追加换行符 \n。如果需要换行符则需要手动添加
    let result1 = std::io::stdout().write("面向加薪学习".as_bytes()).unwrap();
    println!("写入的字节数为：{}", result1);

    // std::env::args() 函数返回所有的命令行参数，返回的结果包含了程序名。
    // 如果要传递多个参数，多个参数之间必须使用 空格（ ' ' ） 分隔。
    // 如果参数里有空格，则参数必须使用 双引号（"）包起来。
    let input_args = std::env::args();
    for arg in input_args{
        println!("命令行参数：{}", arg);
    }
}