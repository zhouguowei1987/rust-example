fn main(){
    // 左区间..右区间，这是一个左闭右开的区间，1..5，那就只包含 1,2,3,4
    for num in 1..5{
        println!("num is {}", num);
    }

    // 可以使用 a..=b 表示两端都包含在内的范围
    for num in 1..=5{
        println!("num is {}", num);
    }

    let study_list = vec![
        "《Go语言极简一本通》",
        "Go语言微服务架构核心22讲",
        "从0到Go语言微服务架构师",
    ];
    // iter - 在每次迭代中借用集合中的一个元素。这样集合本身不会被改变，循环之后仍可以使用。
    for name in study_list.iter(){
        match name{
            &"从0到Go语言微服务架构师" => println!("恭喜你进阶到第三阶段-{}!", name),
            _ => println!("学习: {}", name),
        }
    }
    println!("study_list: {:?}", study_list);

    let study_list2 = vec![
        "《Go语言极简一本通》",
        "Go语言微服务架构核心22讲",
        "从0到Go语言微服务架构师",
    ];
    // into_iter - 会消耗集合。在每次迭代中，集合中的数据本身会被提供。一旦集合被消耗了，之后就无法再使用了，因为它已经在循环中被 “移除”（move）了。
    for name in study_list2.into_iter(){
        match name{
            "从0到Go语言微服务架构师" => println!("恭喜你进阶到第三阶段-{}!", name),
            _ => println!("学习: {}", name),
        }
    }
    // println!("study_list2: {:?}", study_list2);

    let mut study_list3 = vec![
        "《Go语言极简一本通》",
        "Go语言微服务架构核心22讲",
        "从0到Go语言微服务架构师",
    ];
    // iter_mut - 可变地（mutably）借用集合中的每个元素，从而允许集合被就地修改。
    for name in study_list3.iter_mut(){
        *name = match name{
            &mut "从0到Go语言微服务架构师" => "恭喜你进阶到第三阶段---从0到Go语言微服务架构师",
            _ => *name,
        }
    }
    println!("study_list3: {:?}", study_list3);

    let mut num = 1;
    while num < 20{
        println!("num is {}", num);
        num = num * 2;
    }

    let mut num = 1;
    loop{
        if num > 20{
            break;
        }
        println!("num is {}", num);
        num = num * 3;
    }
}