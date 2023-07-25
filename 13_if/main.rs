fn main(){
    let total:f32 = 366.00;
    if total > 200.00 && total < 500.00{
        println!("打9折，{}", total * 0.9)
    }else if total > 500.00 {
        println!("打8折，{}", total * 0.8)
    }else{
        println!("无折扣优惠，{}", total)
    }

    let code = "10010";
     let choose = match code {
         "10010" => "联通",
         "10086" => "移动",
         _ => "Unknown"
     };
    println!("{}", choose)
}