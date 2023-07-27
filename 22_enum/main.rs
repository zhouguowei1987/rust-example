// #[derive(Debug)] 注解的作用，就是让 派生自Debug`
#[derive(Debug)]
enum RoadMap{
    Go语言极简一本通,
    Go语言微服务架构核心22讲,
    从0到Go语言微服务架构师,
}

#[derive(Debug)]
enum Option<T>{
    Some(T), // 用于返回一个值
    None, // 用于返回null，用None来替代
}

#[derive(Debug)]
enum StudyRoadMap{
    Name(String),
}
fn main(){
    let level = RoadMap::从0到Go语言微服务架构师;
    println!("level----{:?}", level);

    let p = 66;
    // let p = 666;
    let result = get_discount(p);
    println!("result : {:?}", result);

    print_road_map(RoadMap::Go语言极简一本通);
    print_road_map(RoadMap::Go语言微服务架构核心22讲);
    print_road_map(RoadMap::Go语言微服务架构核心22讲);

    let level3 = StudyRoadMap::Name(String::from("从0到Go语言微服务架构师"));
    match level3 {
        StudyRoadMap::Name(val)=>{
            println!("{:?}", val);
        }
    }
}

fn get_discount(price:i32) -> Option<bool>{
    if price > 100{
        Option::Some(true)
    }else {
        Option::None
    }
}

fn print_road_map(r:RoadMap){
    match r {
        RoadMap::Go语言极简一本通=>{
            println!("Go语言极简一本通");
        },
        RoadMap::Go语言微服务架构核心22讲=>{
            println!("Go语言微服务架构核心22讲");
        },
        RoadMap::从0到Go语言微服务架构师=>{
            println!("从0到Go语言微服务架构师");
        },
    }
}