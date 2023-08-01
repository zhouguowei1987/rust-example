// use std::fs as fs;
fn main(){
    // 打开文件
    // 静态方法，以 只读 模式打开文件
    let file = std::fs::File::open("data.txt").unwrap();
    println!("文件打开成功\n : {:?}", file);

    // 创建文件
    // 静态方法，以 可写 模式打开文件。
    // 如果文件存在则清空旧内容 如果文件不存在则新建
    let file = std::fs::File::create("data2.txt").expect("创建失败");
    println!("文件创建成功\n : {:?}", file);

    // 删除文件
    // 从文件系统中删除某个文件
    std::fs::remove_file("data2.txt").expect("无法删除文件");
    println!("删除文件成功\n : {:?}", file);

    // 追加内容
}