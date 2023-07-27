#[derive(Debug)]
struct Study {
    name: String,
    target: String,
    spend: i32,
}
impl Study{
    // 结构体普通方法
    // self 是“自己”的意思，&self 表示当前结构体的实例。
    // &self 也是结构体普通方法固定的第一个参数，其他参数可选
    // 结构体方法的作用域仅限于结构体内部
    fn get_spend(&self) -> i32{
        return self.spend;
    }

    // 结构体静态方法
    // 调用方式 结构体名称::方法名(参数列表)
    fn get_instance_another(name:String, target:String, spend:i32) -> Study{
        return Study{
            name,
            target,
            spend,
        }
    }
}
fn main(){
    let s = Study{
        name: String::from("从0到Go语言微服务架构师"),
        target: String::from("全面掌握Go语言微服务落地，代码级一次性解决微服务和分布式系统。"),
        spend: 3,
    };
    println!("s : {:?}", s);

    println!("s.name : {}",s.name);

    show(s);

    let mut s2 = Study{
        name: String::from("从0到Go语言微服务架构师"),
        target: String::from("全面掌握Go语言微服务落地，代码级一次性解决微服务和分布式系统。"),
        spend: 3,
    };
    s2.spend = 7;
    println!("s2 : {:?}", s2);

    let s3 = get_instance(
        String::from("Go语言极简一本通"),
        String::from("学习Go语言语法，并且完成一个单体服务"),
        5,
    );
    println!("s3 : {:?}", s3);
    println!("s3.spend : {}", s3.get_spend());

    let s4 = Study::get_instance_another(
        String::from("Go语言极简一本通"),
        String::from("学习Go语言语法，并且完成一个单体服务"),
        6,
    );
    println!("s4 : {:?}", s4);
}

fn show(s:Study){
    println!(
        "name is :{} target is {} spend is{}",
        s.name,
        s.target,
        s.spend
    );
}

fn get_instance(name:String, target:String, spend:i32) -> Study{
    return Study{
        name,
        target,
        spend,
    }
}