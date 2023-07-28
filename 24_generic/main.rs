// 泛型结构体
struct Data<T>{
    value: T,
}

struct Book{
    name: String,
    id: u32,
    author: String,
}
// 可以把这个特质（traits）对标其他语言的接口，都是对行为的抽象。
// 使用 trait关键字用来定义。特质，可以包含具体的方法，也可以包含抽象的方法。
trait ShowBook{
    // 没有任何实现的虚方法
    fn show(&self);
}
// Rust 使用 impl for 为每个结构体实现某个特质。
// impl 是 implement 的缩写
impl ShowBook for Book{
    fn show(&self){
        println!(
            "id : {}, author : {}, name : {}",
            self.id,
            self.author,
            self.name
        )
    }
}
fn main(){
    let mut v:Vec<i32> = vec![1, 2, 3];
    // v.push("4");
    v.push(4);
    println!("v : {:?}", v);

    let t:Data<i32> = Data{
        value: 100,
    };
    println!("t.value : {}", t.value);

    let t:Data<f64> = Data{
        value: 66.00
    };
    println!("t.value : {}", t.value);
    
    let book = Book{
        name: String::from("Go语言极简一本通"),
        id: 1,
        author: String::from("喜欢"),
    };
    book.show();
}