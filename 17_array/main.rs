fn main(){
    let mut arr1:[&str;3] = ["Go语言极简一本通","Go语言微服务架构核心22讲","从0到Go语言微服务架构师"];
    println!("arr1 length : {}", arr1.len());

    for item1 in arr1{
        println!("arr1 item1 : {}", item1);
    }
    println!("arr1 = {:?}", arr1);

    for item2 in arr1.iter(){
        println!("arr1 item2 : {}", item2);
    }
    println!("arr1 = {:?}", arr1);

    arr1[0] = "haha";
    println!("arr1 = {:?}", arr1);

    let arr2 = ["Go语言极简一本通","Go语言微服务架构核心22讲","从0到Go语言微服务架构师"];
    println!("调用show_arr之前，arr2 = {:?}", arr2);
    show_arr(arr2);
    println!("调用show_arr之后，arr2 = {:?}", arr2);

    let mut arr3:[&str;3] = ["Go语言极简一本通","Go语言微服务架构核心22讲","从0到Go语言微服务架构师"];
    println!("调用modify_arr之前，arr3 = {:?}", arr3);
    modify_arr(&mut arr3);
    println!("调用modify_arr之后，arr3 = {:?}", arr3);
}

fn show_arr(arr:[&str;3]){
    let l = arr.len();
    for i in 0..l{
        println!("arr2 {} : {}", i, arr[i]);
    }
}

fn modify_arr(arr:&mut [&str;3]){
    let l = arr.len();
    for i in 0..l{
        arr[i] = "";
    }
}

