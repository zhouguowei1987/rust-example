fn main(){
    println!("{}",std::u128::MAX);
    println!("{}", std::i128::MAX);
    println!("{}", std::i128::MIN);

    let price = 100;
    let price2:u32 = 200;
    let price3:i32 = -300;
    let price4:isize = 400;
    let price5:usize = 500;

    println!("price is {}", price);

    println!("price2 is {} and price3 is {}", price2, price3);
    println!("price4 is {} and price5 is {}", price4, price5);

    // let price6:i32 = 66.66;

    // let price7:i8 = 192;

}